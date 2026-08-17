use std::sync::Mutex;

struct Account {
    id: u64,
    balance: Mutex<u64>,
}

#[derive(Debug, PartialEq, Eq)]
enum TransferError {
    SameAccount,
    Insufficient,
}

fn transfer(from: &Account, to: &Account, amount: u64) -> Result<(), TransferError> {
    if from.id == to.id {
        // Sin esta guarda, la doble adquisición del mismo mutex
        // bloquearía para siempre: el Mutex de std no es reentrante.
        return Err(TransferError::SameAccount);
    }

    // Regla global: bloquear siempre primero la cuenta de id menor.
    let (first, second) = if from.id < to.id { (from, to) } else { (to, from) };
    let mut first_guard = first.balance.lock().unwrap();
    let mut second_guard = second.balance.lock().unwrap();

    let (from_balance, to_balance) = if from.id < to.id {
        (&mut *first_guard, &mut *second_guard)
    } else {
        (&mut *second_guard, &mut *first_guard)
    };

    if *from_balance < amount {
        return Err(TransferError::Insufficient);
    }
    *from_balance -= amount;
    *to_balance += amount;
    Ok(())
}
