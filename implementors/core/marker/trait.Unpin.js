(function() {var implementors = {};
implementors["openmls"] = [{"text":"impl Unpin for ErrorString","synthetic":true,"types":[]},{"text":"impl Unpin for ErrorPayload","synthetic":true,"types":[]},{"text":"impl Unpin for HKDFError","synthetic":true,"types":[]},{"text":"impl Unpin for CryptoError","synthetic":true,"types":[]},{"text":"impl Unpin for CiphersuiteName","synthetic":true,"types":[]},{"text":"impl Unpin for SignatureScheme","synthetic":true,"types":[]},{"text":"impl Unpin for HpkeCiphertext","synthetic":true,"types":[]},{"text":"impl Unpin for KdfLabel","synthetic":true,"types":[]},{"text":"impl Unpin for Secret","synthetic":true,"types":[]},{"text":"impl Unpin for AeadKey","synthetic":true,"types":[]},{"text":"impl Unpin for ReuseGuard","synthetic":true,"types":[]},{"text":"impl Unpin for AeadNonce","synthetic":true,"types":[]},{"text":"impl Unpin for Signature","synthetic":true,"types":[]},{"text":"impl Unpin for SignaturePrivateKey","synthetic":true,"types":[]},{"text":"impl Unpin for SignaturePublicKey","synthetic":true,"types":[]},{"text":"impl Unpin for SignatureKeypair","synthetic":true,"types":[]},{"text":"impl Unpin for Ciphersuite","synthetic":true,"types":[]},{"text":"impl Unpin for CodecError","synthetic":true,"types":[]},{"text":"impl Unpin for VecSize","synthetic":true,"types":[]},{"text":"impl Unpin for Cursor","synthetic":true,"types":[]},{"text":"impl Unpin for ConfigError","synthetic":true,"types":[]},{"text":"impl Unpin for CONFIG","synthetic":true,"types":[]},{"text":"impl Unpin for Constants","synthetic":true,"types":[]},{"text":"impl Unpin for PersistentConfig","synthetic":true,"types":[]},{"text":"impl Unpin for Config","synthetic":true,"types":[]},{"text":"impl Unpin for ProtocolVersion","synthetic":true,"types":[]},{"text":"impl Unpin for CredentialError","synthetic":true,"types":[]},{"text":"impl Unpin for CredentialType","synthetic":true,"types":[]},{"text":"impl Unpin for Certificate","synthetic":true,"types":[]},{"text":"impl Unpin for MLSCredentialType","synthetic":true,"types":[]},{"text":"impl Unpin for Credential","synthetic":true,"types":[]},{"text":"impl Unpin for BasicCredential","synthetic":true,"types":[]},{"text":"impl Unpin for CredentialBundle","synthetic":true,"types":[]},{"text":"impl Unpin for CapabilitiesExtension","synthetic":true,"types":[]},{"text":"impl Unpin for ExtensionError","synthetic":true,"types":[]},{"text":"impl Unpin for LifetimeExtensionError","synthetic":true,"types":[]},{"text":"impl Unpin for CapabilitiesExtensionError","synthetic":true,"types":[]},{"text":"impl Unpin for KeyPackageIdError","synthetic":true,"types":[]},{"text":"impl Unpin for ParentHashError","synthetic":true,"types":[]},{"text":"impl Unpin for RatchetTreeError","synthetic":true,"types":[]},{"text":"impl Unpin for InvalidExtensionError","synthetic":true,"types":[]},{"text":"impl Unpin for KeyIDExtension","synthetic":true,"types":[]},{"text":"impl Unpin for LifetimeExtension","synthetic":true,"types":[]},{"text":"impl Unpin for ParentHashExtension","synthetic":true,"types":[]},{"text":"impl Unpin for RatchetTreeExtension","synthetic":true,"types":[]},{"text":"impl Unpin for ExtensionType","synthetic":true,"types":[]},{"text":"impl Unpin for ExtensionStruct","synthetic":true,"types":[]},{"text":"impl Unpin for MLSCiphertext","synthetic":true,"types":[]},{"text":"impl Unpin for MLSSenderData","synthetic":true,"types":[]},{"text":"impl Unpin for MLSSenderDataAAD","synthetic":true,"types":[]},{"text":"impl Unpin for MLSCiphertextContent","synthetic":true,"types":[]},{"text":"impl Unpin for MLSCiphertextContentAAD","synthetic":true,"types":[]},{"text":"impl Unpin for MLSPlaintextError","synthetic":true,"types":[]},{"text":"impl Unpin for MLSCiphertextError","synthetic":true,"types":[]},{"text":"impl Unpin for VerificationError","synthetic":true,"types":[]},{"text":"impl Unpin for MLSPlaintext","synthetic":true,"types":[]},{"text":"impl Unpin for ContentType","synthetic":true,"types":[]},{"text":"impl Unpin for MLSPlaintextContentType","synthetic":true,"types":[]},{"text":"impl Unpin for Mac","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Unpin for MLSPlaintextTBMPayload&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Unpin for MembershipTag","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Unpin for MLSPlaintextTBS&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Unpin for MLSPlaintextTBSPayload","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Unpin for MLSPlaintextCommitContent&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Unpin for MLSPlaintextCommitAuthData&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Unpin for SenderType","synthetic":true,"types":[]},{"text":"impl Unpin for Sender","synthetic":true,"types":[]},{"text":"impl Unpin for GroupError","synthetic":true,"types":[]},{"text":"impl Unpin for WelcomeError","synthetic":true,"types":[]},{"text":"impl Unpin for ApplyCommitError","synthetic":true,"types":[]},{"text":"impl Unpin for CreateCommitError","synthetic":true,"types":[]},{"text":"impl Unpin for ExporterError","synthetic":true,"types":[]},{"text":"impl Unpin for PskError","synthetic":true,"types":[]},{"text":"impl Unpin for ManagedGroupCallbacks","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Unpin for Removal&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Unpin for HandshakeMessageFormat","synthetic":true,"types":[]},{"text":"impl Unpin for ManagedGroupConfig","synthetic":true,"types":[]},{"text":"impl Unpin for UpdatePolicy","synthetic":true,"types":[]},{"text":"impl Unpin for ManagedGroupError","synthetic":true,"types":[]},{"text":"impl Unpin for EmptyInputError","synthetic":true,"types":[]},{"text":"impl Unpin for UseAfterEviction","synthetic":true,"types":[]},{"text":"impl Unpin for PendingProposalsError","synthetic":true,"types":[]},{"text":"impl Unpin for InvalidMessageError","synthetic":true,"types":[]},{"text":"impl Unpin for ResumptionSecretStore","synthetic":true,"types":[]},{"text":"impl Unpin for SerializedManagedGroup","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Unpin for ManagedGroup&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Unpin for MLSMessage","synthetic":true,"types":[]},{"text":"impl Unpin for PlaintextSecret","synthetic":true,"types":[]},{"text":"impl Unpin for MlsGroup","synthetic":true,"types":[]},{"text":"impl Unpin for GroupId","synthetic":true,"types":[]},{"text":"impl Unpin for GroupEpoch","synthetic":true,"types":[]},{"text":"impl Unpin for GroupContext","synthetic":true,"types":[]},{"text":"impl Unpin for GroupConfig","synthetic":true,"types":[]},{"text":"impl Unpin for KeyPackageError","synthetic":true,"types":[]},{"text":"impl Unpin for KeyPackage","synthetic":true,"types":[]},{"text":"impl Unpin for KeyPackageBundle","synthetic":true,"types":[]},{"text":"impl Unpin for ProposalQueueError","synthetic":true,"types":[]},{"text":"impl Unpin for ProposalOrRefTypeError","synthetic":true,"types":[]},{"text":"impl Unpin for QueuedProposalError","synthetic":true,"types":[]},{"text":"impl Unpin for ProposalType","synthetic":true,"types":[]},{"text":"impl Unpin for ProposalOrRefType","synthetic":true,"types":[]},{"text":"impl Unpin for ProposalOrRef","synthetic":true,"types":[]},{"text":"impl Unpin for Proposal","synthetic":true,"types":[]},{"text":"impl Unpin for ProposalReference","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Unpin for QueuedProposal&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Unpin for ProposalQueue&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Unpin for AddProposal","synthetic":true,"types":[]},{"text":"impl Unpin for UpdateProposal","synthetic":true,"types":[]},{"text":"impl Unpin for RemoveProposal","synthetic":true,"types":[]},{"text":"impl Unpin for PreSharedKeyProposal","synthetic":true,"types":[]},{"text":"impl Unpin for ReInitProposal","synthetic":true,"types":[]},{"text":"impl Unpin for Welcome","synthetic":true,"types":[]},{"text":"impl Unpin for EncryptedGroupSecrets","synthetic":true,"types":[]},{"text":"impl Unpin for Commit","synthetic":true,"types":[]},{"text":"impl Unpin for ConfirmationTag","synthetic":true,"types":[]},{"text":"impl Unpin for GroupInfo","synthetic":true,"types":[]},{"text":"impl Unpin for PathSecret","synthetic":true,"types":[]},{"text":"impl Unpin for GroupSecrets","synthetic":true,"types":[]},{"text":"impl Unpin for PublicGroupState","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Unpin for PublicGroupStateTBS&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Unpin for ErrorState","synthetic":true,"types":[]},{"text":"impl Unpin for KeyScheduleError","synthetic":true,"types":[]},{"text":"impl Unpin for PskSecretError","synthetic":true,"types":[]},{"text":"impl Unpin for PSKType","synthetic":true,"types":[]},{"text":"impl Unpin for ExternalPsk","synthetic":true,"types":[]},{"text":"impl Unpin for ExternalPskBundle","synthetic":true,"types":[]},{"text":"impl Unpin for ReinitPsk","synthetic":true,"types":[]},{"text":"impl Unpin for BranchPsk","synthetic":true,"types":[]},{"text":"impl Unpin for Psk","synthetic":true,"types":[]},{"text":"impl Unpin for PreSharedKeyID","synthetic":true,"types":[]},{"text":"impl Unpin for PreSharedKeys","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Unpin for PskLabel&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Unpin for PskSecret","synthetic":true,"types":[]},{"text":"impl Unpin for CommitSecret","synthetic":true,"types":[]},{"text":"impl Unpin for InitSecret","synthetic":true,"types":[]},{"text":"impl Unpin for JoinerSecret","synthetic":true,"types":[]},{"text":"impl Unpin for State","synthetic":true,"types":[]},{"text":"impl Unpin for KeySchedule","synthetic":true,"types":[]},{"text":"impl Unpin for IntermediateSecret","synthetic":true,"types":[]},{"text":"impl Unpin for WelcomeSecret","synthetic":true,"types":[]},{"text":"impl Unpin for EpochSecret","synthetic":true,"types":[]},{"text":"impl Unpin for EncryptionSecret","synthetic":true,"types":[]},{"text":"impl Unpin for ExporterSecret","synthetic":true,"types":[]},{"text":"impl Unpin for AuthenticationSecret","synthetic":true,"types":[]},{"text":"impl Unpin for ExternalSecret","synthetic":true,"types":[]},{"text":"impl Unpin for ConfirmationKey","synthetic":true,"types":[]},{"text":"impl Unpin for MembershipKey","synthetic":true,"types":[]},{"text":"impl Unpin for ResumptionSecret","synthetic":true,"types":[]},{"text":"impl Unpin for SenderDataSecret","synthetic":true,"types":[]},{"text":"impl Unpin for EpochSecrets","synthetic":true,"types":[]},{"text":"impl Unpin for TreeError","synthetic":true,"types":[]},{"text":"impl Unpin for ParentHashError","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Unpin for ParentHashInput&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Unpin for LeafNodeHashInput&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Unpin for ParentNodeTreeHashInput&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Unpin for NodeIndex","synthetic":true,"types":[]},{"text":"impl Unpin for LeafIndex","synthetic":true,"types":[]},{"text":"impl Unpin for NodeType","synthetic":true,"types":[]},{"text":"impl Unpin for Node","synthetic":true,"types":[]},{"text":"impl Unpin for ParentNode","synthetic":true,"types":[]},{"text":"impl Unpin for PathKeys","synthetic":true,"types":[]},{"text":"impl Unpin for PrivateTree","synthetic":true,"types":[]},{"text":"impl Unpin for SecretTreeError","synthetic":true,"types":[]},{"text":"impl Unpin for SecretType","synthetic":true,"types":[]},{"text":"impl Unpin for TreeContext","synthetic":true,"types":[]},{"text":"impl Unpin for SecretTreeNode","synthetic":true,"types":[]},{"text":"impl Unpin for SecretTree","synthetic":true,"types":[]},{"text":"impl Unpin for SenderRatchet","synthetic":true,"types":[]},{"text":"impl Unpin for TreeMathError","synthetic":true,"types":[]},{"text":"impl Unpin for RatchetTree","synthetic":true,"types":[]},{"text":"impl Unpin for ApplyProposalsValues","synthetic":true,"types":[]},{"text":"impl Unpin for UpdatePathNode","synthetic":true,"types":[]},{"text":"impl Unpin for UpdatePath","synthetic":true,"types":[]}];
implementors["test_macros"] = [{"text":"impl Unpin for TestInput","synthetic":true,"types":[]}];
implementors["tls_codec"] = [{"text":"impl&lt;T&gt; Unpin for TlsVecU8&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for TlsVecU16&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for TlsVecU32&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Unpin for Error","synthetic":true,"types":[]},{"text":"impl Unpin for Cursor","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()