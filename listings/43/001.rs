use std::future::Future;

#[derive(Clone)]
struct Account {
    balance: u64,
}

#[derive(Debug)]
enum AccountError {
    ZeroAmount,
    Overflow,
}

impl Account {
    fn deposit(&mut self, amount: u64) -> Result<u64, AccountError> {
        if amount == 0 {
            return Err(AccountError::ZeroAmount);
        }
        self.balance = self.balance.checked_add(amount).ok_or(AccountError::Overflow)?;
        Ok(self.balance)
    }
}

trait AccountRepository {
    type Error;

    fn load(&self) -> impl Future<Output = Result<Account, Self::Error>> + Send;
    fn save(&self, account: Account) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

#[derive(Debug)]
enum DepositError<E> {
    Load(E),
    Domain(AccountError),
    Save(E),
}

async fn deposit<R>(repository: &R, amount: u64) -> Result<u64, DepositError<R::Error>>
where
    R: AccountRepository + Sync,
{
    let mut account = repository.load().await.map_err(DepositError::Load)?;
    let balance = account.deposit(amount).map_err(DepositError::Domain)?;
    repository.save(account).await.map_err(DepositError::Save)?;
    Ok(balance)
}
