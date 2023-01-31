use openmls_traits::{types::Ciphersuite, OpenMlsCryptoProvider};
use std::io::Write;
use tls_codec::{Deserialize, Serialize, Size, TlsDeserialize, TlsSerialize, TlsSize};

use super::{
    codec::deserialize_ciphertext_content,
    mls_auth_content::{
        AuthenticatedContent, FramedContentAuthData, VerifiableAuthenticatedContent,
    },
    mls_content::{ContentType, FramedContentBody},
};

use crate::{
    binary_tree::array_representation::LeafNodeIndex,
    error::LibraryError,
    framing::mls_content::FramedContent,
    tree::{
        index::SecretTreeLeafIndex, secret_tree::SecretType,
        sender_ratchet::SenderRatchetConfiguration,
    },
};

use super::*;

/// `PrivateMessage` is the framing struct for an encrypted `PublicMessage`.
/// This message format is meant to be sent to and received from the Delivery
/// Service.
///
/// ```c
/// // draft-ietf-mls-protocol-17
/// struct {
///     opaque group_id<V>;
///     uint64 epoch;
///     ContentType content_type;
///     opaque authenticated_data<V>;
///     opaque encrypted_sender_data<V>;
///     opaque ciphertext<V>;
/// } PrivateMessage;
/// ```
#[derive(Debug, PartialEq, Eq, Clone, TlsSerialize, TlsSize, TlsDeserialize)]
pub struct PrivateMessage {
    group_id: GroupId,
    epoch: GroupEpoch,
    content_type: ContentType,
    authenticated_data: VLBytes,
    encrypted_sender_data: VLBytes,
    ciphertext: VLBytes,
}

pub(crate) struct MlsMessageHeader {
    pub(crate) group_id: GroupId,
    pub(crate) epoch: GroupEpoch,
    pub(crate) sender: LeafNodeIndex,
}

impl PrivateMessage {
    #[cfg(test)]
    pub(crate) fn new(
        group_id: GroupId,
        epoch: GroupEpoch,
        content_type: ContentType,
        authenticated_data: VLBytes,
        encrypted_sender_data: VLBytes,
        ciphertext: VLBytes,
    ) -> Self {
        Self {
            group_id,
            epoch,
            content_type,
            authenticated_data,
            encrypted_sender_data,
            ciphertext,
        }
    }

    /// Try to create a new `PrivateMessage` from an `AuthenticatedContent`.
    ///
    /// TODO #1148: Refactor theses constructors to avoid test code in main and
    /// to avoid validation using a special feature flag.
    pub(crate) fn try_from_authenticated_content(
        public_message: &AuthenticatedContent,
        ciphersuite: Ciphersuite,
        backend: &impl OpenMlsCryptoProvider,
        message_secrets: &mut MessageSecrets,
        padding_size: usize,
    ) -> Result<PrivateMessage, MessageEncryptionError> {
        log::debug!("PrivateMessage::try_from_authenticated_content");
        log::trace!("  ciphersuite: {}", ciphersuite);
        // Check the message has the correct wire format
        if public_message.wire_format() != WireFormat::PrivateMessage {
            return Err(MessageEncryptionError::WrongWireFormat);
        }
        Self::encrypt_content(
            None,
            public_message,
            ciphersuite,
            backend,
            message_secrets,
            padding_size,
        )
    }

    #[cfg(test)]
    pub(crate) fn encrypt_without_check(
        public_message: &AuthenticatedContent,
        ciphersuite: Ciphersuite,
        backend: &impl OpenMlsCryptoProvider,
        message_secrets: &mut MessageSecrets,
        padding_size: usize,
    ) -> Result<PrivateMessage, MessageEncryptionError> {
        Self::encrypt_content(
            None,
            public_message,
            ciphersuite,
            backend,
            message_secrets,
            padding_size,
        )
    }

    #[cfg(test)]
    pub(crate) fn encrypt_with_different_header(
        public_message: &AuthenticatedContent,
        ciphersuite: Ciphersuite,
        backend: &impl OpenMlsCryptoProvider,
        header: MlsMessageHeader,
        message_secrets: &mut MessageSecrets,
        padding_size: usize,
    ) -> Result<PrivateMessage, MessageEncryptionError> {
        Self::encrypt_content(
            Some(header),
            public_message,
            ciphersuite,
            backend,
            message_secrets,
            padding_size,
        )
    }

    /// Internal function to encrypt content. The extra message header is only used
    /// for tests. Otherwise, the data from the given `AuthenticatedContent` is used.
    fn encrypt_content(
        test_header: Option<MlsMessageHeader>,
        public_message: &AuthenticatedContent,
        ciphersuite: Ciphersuite,
        backend: &impl OpenMlsCryptoProvider,
        message_secrets: &mut MessageSecrets,
        padding_size: usize,
    ) -> Result<PrivateMessage, MessageEncryptionError> {
        let sender_index = if let Some(index) = public_message.sender().as_member() {
            index
        } else {
            return Err(MessageEncryptionError::SenderError(SenderError::NotAMember));
        };
        // Take the provided header only if one is given and if this is indeed a test.
        let header = match test_header {
            Some(header) if cfg!(any(feature = "test-utils", test)) => header,
            _ => MlsMessageHeader {
                group_id: public_message.group_id().clone(),
                epoch: public_message.epoch(),
                sender: sender_index,
            },
        };
        // Serialize the content AAD
        let private_message_content_aad = PrivateContentAad {
            group_id: header.group_id.clone(),
            epoch: header.epoch,
            content_type: public_message.content().content_type(),
            authenticated_data: TlsByteSliceU32(public_message.authenticated_data()),
        };
        let private_message_content_aad_bytes = private_message_content_aad
            .tls_serialize_detached()
            .map_err(LibraryError::missing_bound_check)?;
        // Extract generation and key material for encryption
        let secret_type = SecretType::from(&public_message.content().content_type());
        let (generation, (ratchet_key, ratchet_nonce)) = message_secrets
            .secret_tree_mut()
            // Even in tests we want to use the real sender index, so we have a key to encrypt.
            .secret_for_encryption(ciphersuite, backend, sender_index.into(), secret_type)?;
        // Sample reuse guard uniformly at random.
        let reuse_guard: ReuseGuard =
            ReuseGuard::try_from_random(backend).map_err(LibraryError::unexpected_crypto_error)?;
        // Prepare the nonce by xoring with the reuse guard.
        let prepared_nonce = ratchet_nonce.xor_with_reuse_guard(&reuse_guard);
        // Encrypt the payload
        let ciphertext = ratchet_key
            .aead_seal(
                backend,
                &Self::encode_padded_ciphertext_content_detached(
                    public_message,
                    padding_size,
                    ciphersuite.mac_length(),
                )
                .map_err(LibraryError::missing_bound_check)?,
                &private_message_content_aad_bytes,
                &prepared_nonce,
            )
            .map_err(LibraryError::unexpected_crypto_error)?;
        // Derive the sender data key from the key schedule using the ciphertext.
        let sender_data_key = message_secrets
            .sender_data_secret()
            .derive_aead_key(backend, &ciphertext)
            .map_err(LibraryError::unexpected_crypto_error)?;
        // Derive initial nonce from the key schedule using the ciphertext.
        let sender_data_nonce = message_secrets
            .sender_data_secret()
            .derive_aead_nonce(ciphersuite, backend, &ciphertext)
            .map_err(LibraryError::unexpected_crypto_error)?;
        // Compute sender data nonce by xoring reuse guard and key schedule
        // nonce as per spec.
        let mls_sender_data_aad = MlsSenderDataAad::new(
            header.group_id.clone(),
            header.epoch,
            public_message.content().content_type(),
        );
        // Serialize the sender data AAD
        let mls_sender_data_aad_bytes = mls_sender_data_aad
            .tls_serialize_detached()
            .map_err(LibraryError::missing_bound_check)?;
        let sender_data = MlsSenderData::from_sender(
            // XXX: #106 This will fail for messages with a non-member sender.
            header.sender,
            generation,
            reuse_guard,
        );
        // Encrypt the sender data
        let encrypted_sender_data = sender_data_key
            .aead_seal(
                backend,
                &sender_data
                    .tls_serialize_detached()
                    .map_err(LibraryError::missing_bound_check)?,
                &mls_sender_data_aad_bytes,
                &sender_data_nonce,
            )
            .map_err(LibraryError::unexpected_crypto_error)?;
        Ok(PrivateMessage {
            group_id: header.group_id.clone(),
            epoch: header.epoch,
            content_type: public_message.content().content_type(),
            authenticated_data: public_message.authenticated_data().into(),
            encrypted_sender_data: encrypted_sender_data.into(),
            ciphertext: ciphertext.into(),
        })
    }

    /// Decrypt the sender data from this [`PrivateMessage`].
    pub(crate) fn sender_data(
        &self,
        message_secrets: &MessageSecrets,
        backend: &impl OpenMlsCryptoProvider,
        ciphersuite: Ciphersuite,
    ) -> Result<MlsSenderData, MessageDecryptionError> {
        log::debug!("Decrypting PrivateMessage");
        // Derive key from the key schedule using the ciphertext.
        let sender_data_key = message_secrets
            .sender_data_secret()
            .derive_aead_key(backend, self.ciphertext.as_slice())
            .map_err(LibraryError::unexpected_crypto_error)?;
        // Derive initial nonce from the key schedule using the ciphertext.
        let sender_data_nonce = message_secrets
            .sender_data_secret()
            .derive_aead_nonce(ciphersuite, backend, self.ciphertext.as_slice())
            .map_err(LibraryError::unexpected_crypto_error)?;
        // Serialize sender data AAD
        let mls_sender_data_aad =
            MlsSenderDataAad::new(self.group_id.clone(), self.epoch, self.content_type);
        let mls_sender_data_aad_bytes = mls_sender_data_aad
            .tls_serialize_detached()
            .map_err(LibraryError::missing_bound_check)?;
        // Decrypt sender data
        let sender_data_bytes = sender_data_key
            .aead_open(
                backend,
                self.encrypted_sender_data.as_slice(),
                &mls_sender_data_aad_bytes,
                &sender_data_nonce,
            )
            .map_err(|_| {
                log::error!("Sender data decryption error");
                MessageDecryptionError::AeadError
            })?;
        log::trace!("  Successfully decrypted sender data.");
        MlsSenderData::tls_deserialize(&mut sender_data_bytes.as_slice())
            .map_err(|_| MessageDecryptionError::MalformedContent)
    }

    /// Decrypt this [`PrivateMessage`] and return the [`PrivateContentTbe`].
    #[inline]
    fn decrypt(
        &self,
        backend: &impl OpenMlsCryptoProvider,
        ratchet_key: AeadKey,
        ratchet_nonce: &AeadNonce,
    ) -> Result<PrivateContentTbe, MessageDecryptionError> {
        // Serialize content AAD
        let private_message_content_aad_bytes = PrivateContentAad {
            group_id: self.group_id.clone(),
            epoch: self.epoch,
            content_type: self.content_type,
            authenticated_data: TlsByteSliceU32(self.authenticated_data.as_slice()),
        }
        .tls_serialize_detached()
        .map_err(LibraryError::missing_bound_check)?;
        // Decrypt payload
        let private_message_content_bytes = ratchet_key
            .aead_open(
                backend,
                self.ciphertext.as_slice(),
                &private_message_content_aad_bytes,
                ratchet_nonce,
            )
            .map_err(|_| {
                log::error!("  Ciphertext decryption error");
                MessageDecryptionError::AeadError
            })?;
        log_content!(
            trace,
            "  Successfully decrypted PublicMessage bytes: {:x?}",
            private_message_content_bytes
        );
        deserialize_ciphertext_content(
            &mut private_message_content_bytes.as_slice(),
            self.content_type(),
        )
        .map_err(|_| MessageDecryptionError::MalformedContent)
    }

    /// This function decrypts a [`PrivateMessage`] into a [`VerifiableAuthenticatedContent`].
    /// In order to get an [`FramedContent`] the result must be verified.
    pub(crate) fn to_verifiable_content(
        &self,
        ciphersuite: Ciphersuite,
        backend: &impl OpenMlsCryptoProvider,
        message_secrets: &mut MessageSecrets,
        sender_index: SecretTreeLeafIndex,
        sender_ratchet_configuration: &SenderRatchetConfiguration,
        sender_data: MlsSenderData,
    ) -> Result<VerifiableAuthenticatedContent, MessageDecryptionError> {
        let secret_type = SecretType::from(&self.content_type);
        // Extract generation and key material for encryption
        let (ratchet_key, ratchet_nonce) = message_secrets
            .secret_tree_mut()
            .secret_for_decryption(
                ciphersuite,
                backend,
                sender_index,
                secret_type,
                sender_data.generation,
                sender_ratchet_configuration,
            )
            .map_err(|_| {
                log::error!("  Ciphertext generation out of bounds");
                MessageDecryptionError::GenerationOutOfBound
            })?;
        // Prepare the nonce by xoring with the reuse guard.
        let prepared_nonce = ratchet_nonce.xor_with_reuse_guard(&sender_data.reuse_guard);
        let private_message_content = self.decrypt(backend, ratchet_key, &prepared_nonce)?;

        // Extract sender. The sender type is always of type Member for PrivateMessage.
        let sender = Sender::from_sender_data(sender_data);
        log_content!(
            trace,
            "  Successfully decoded PublicMessage with: {:x?}",
            private_message_content.content
        );

        let verifiable = VerifiableAuthenticatedContent::new(
            WireFormat::PrivateMessage,
            FramedContent {
                group_id: self.group_id.clone(),
                epoch: self.epoch,
                sender,
                authenticated_data: self.authenticated_data.clone(),
                body: private_message_content.content,
            },
            Some(message_secrets.serialized_context().to_vec()),
            private_message_content.auth,
        );
        Ok(verifiable)
    }

    /// Returns `true` if this is a handshake message and `false` otherwise.
    #[cfg(test)]
    pub(crate) fn is_handshake_message(&self) -> bool {
        self.content_type.is_handshake_message()
    }

    /// Encodes the `PrivateContentTbe` struct with padding.
    fn encode_padded_ciphertext_content_detached(
        authenticated_content: &AuthenticatedContent,
        padding_size: usize,
        mac_len: usize,
    ) -> Result<Vec<u8>, tls_codec::Error> {
        let plaintext_length = authenticated_content
            .content()
            .serialized_len_without_type()
            + authenticated_content.auth.tls_serialized_len();

        let padding_length = if padding_size > 0 {
            // Calculate padding block size.
            // Only the AEAD tag is added.
            let padding_offset = plaintext_length + mac_len;
            // Return padding block size
            (padding_size - (padding_offset % padding_size)) % padding_size
        } else {
            0
        };

        // Persist all initial fields manually (avoids cloning them)
        let buffer = &mut Vec::with_capacity(plaintext_length + padding_length);

        // The `content` field is serialized without the `content_type`, which
        // is not part of the struct as per MLS spec.
        authenticated_content
            .content()
            .serialize_without_type(buffer)?;
        authenticated_content.auth.tls_serialize(buffer)?;
        // Note: The `tls_codec::Serialize` implementation for `&[u8]` prepends the length.
        // We do not want this here and thus use the "raw" `write_all` method.
        buffer
            .write_all(&vec![0u8; padding_length])
            .map_err(|_| Error::EncodingError("Failed to write padding.".into()))?;

        Ok(buffer.to_vec())
    }

    /// Get the `group_id` in the `PrivateMessage`.
    pub(crate) fn group_id(&self) -> &GroupId {
        &self.group_id
    }

    /// Get the cipher text bytes as slice.
    #[cfg(test)]
    pub(crate) fn ciphertext(&self) -> &[u8] {
        self.ciphertext.as_slice()
    }

    /// Get the `epoch` in the `PrivateMessage`.
    pub(crate) fn epoch(&self) -> GroupEpoch {
        self.epoch
    }

    /// Get the `content_type` in the `PrivateMessage`.
    pub(crate) fn content_type(&self) -> ContentType {
        self.content_type
    }

    /// Set the ciphertext.
    #[cfg(test)]
    pub(crate) fn set_ciphertext(&mut self, ciphertext: Vec<u8>) {
        self.ciphertext = ciphertext.into();
    }
}

// === Helper structs ===

#[derive(Clone, TlsDeserialize, TlsSerialize, TlsSize)]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct MlsSenderData {
    pub(crate) leaf_index: LeafNodeIndex,
    pub(crate) generation: u32,
    pub(crate) reuse_guard: ReuseGuard,
}

impl MlsSenderData {
    /// Build new [`MlsSenderData`] for a [`Sender`].
    pub(crate) fn from_sender(
        leaf_index: LeafNodeIndex,
        generation: u32,
        reuse_guard: ReuseGuard,
    ) -> Self {
        MlsSenderData {
            leaf_index,
            generation,
            reuse_guard,
        }
    }
}

#[derive(Clone, TlsDeserialize, TlsSerialize, TlsSize)]
pub(crate) struct MlsSenderDataAad {
    pub(crate) group_id: GroupId,
    pub(crate) epoch: GroupEpoch,
    pub(crate) content_type: ContentType,
}

impl MlsSenderDataAad {
    fn new(group_id: GroupId, epoch: GroupEpoch, content_type: ContentType) -> Self {
        Self {
            group_id,
            epoch,
            content_type,
        }
    }

    #[cfg(test)]
    pub fn test_new(group_id: GroupId, epoch: GroupEpoch, content_type: ContentType) -> Self {
        Self::new(group_id, epoch, content_type)
    }
}

/// PrivateContentTbe
///
/// ```c
/// // draft-ietf-mls-protocol-17
/// struct {
///     select (PrivateMessage.content_type) {
///         case application:
///           opaque application_data<V>;
///
///         case proposal:
///           Proposal proposal;
///
///         case commit:
///           Commit commit;
///     }
///
///     FramedContentAuthData auth;
///     opaque padding[length_of_padding];
/// } PrivateContentTbe;
/// ```
#[derive(Debug, Clone)]
pub(crate) struct PrivateContentTbe {
    // The `content` field is serialized and deserialized manually without the
    // `content_type`, which is not part of the struct as per MLS spec. See the
    // implementation of `TlsSerialize` for `PrivateContentTbe`, as well as
    // `deserialize_ciphertext_content`.
    pub(crate) content: FramedContentBody,
    pub(crate) auth: FramedContentAuthData,
    /// Length of the all-zero padding.
    ///
    /// We do not retain any bytes here to avoid the need to
    /// keep track that all of them are zero. Instead, we only
    /// use `length_of_padding` to track the (theoretical) size
    /// of the all-zero byte slice.
    ///
    /// Note, however, that we MUST make sure to (de)serialize these bytes!
    /// Otherwise this mechanism would not make any sense because it would
    /// not add to the ciphertext size to hide the original message length.
    ///
    /// Sadly, we cannot `derive(TlsSerialize, TlsDeserialize)` due to this
    /// "custom" mechanism.
    pub(crate) length_of_padding: usize,
}

#[derive(TlsSerialize, TlsSize)]
pub(crate) struct PrivateContentAad<'a> {
    pub(crate) group_id: GroupId,
    pub(crate) epoch: GroupEpoch,
    pub(crate) content_type: ContentType,
    pub(crate) authenticated_data: TlsByteSliceU32<'a>,
}