use std::future::Future;

#[derive(Clone)]
struct Account {
    balance: u64,
}

#[derive(Debug)]
enum AccountError {
    Overflow,
}

impl Account {
    fn deposit(&mut self, amount: u64) -> Result<u64, AccountError> {
        self.balance = self.balance.checked_add(amount).ok_or(AccountError::Overflow)?;
        Ok(self.balance)
    }
}

trait AccountRepository {
    type Error;

    fn load(&self) -> impl Future<Output = Result<Account, Self::Error>> + Send;
    fn save(&self, account: Account) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

async fn deposit<R>(repository: &R, amount: u64) -> Result<u64, &'static str>
where
    R: AccountRepository + Sync,
{
    let mut account = repository.load().await.map_err(|_| "load")?;
    let balance = account.deposit(amount).map_err(|_| "domain")?;
    repository.save(account).await.map_err(|_| "save")?;
    Ok(balance)
}
