window.SIDEBAR_ITEMS = {"enum":[["ProcessedMessage","A message that has passed all syntax and semantics checks."],["Sender","All possible sender types according to the MLS protocol spec."],["WireFormat","Wire format of MLS messages."]],"mod":[["errors","Framing errors."]],"struct":[["ApplicationMessage","Application message received through a [ProcessedMessage]."],["MlsMessageIn","Unified message type for incoming MLS messages."],["MlsMessageOut","Unified message type for outgoing MLS messages."],["UnverifiedMessage","Partially checked and potentially decrypted message (if it was originally encrypted). Use this to inspect the [`Credential`] of the message sender and the optional `aad` if the original message was encrypted."]]};