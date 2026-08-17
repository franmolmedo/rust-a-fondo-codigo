match error {
    ProtocolError::Timeout => retry(),
    ProtocolError::Rejected => abort(),
    _ => report_unknown(&error), // exigido por non_exhaustive
}
