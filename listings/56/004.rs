fn transfer(
    accounts: &mut [Account],
    from: usize,
    to: usize,
    amount: Money,
) -> Result<(), TransferError>;
