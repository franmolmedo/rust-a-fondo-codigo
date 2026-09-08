use std::sync::Mutex;

struct Account {
    id: u64,
    balance: Mutex<u64>,
}

#[derive(Debug, PartialEq, Eq)]
enum TransferError {
    SameAccount,
    Insufficient,
    BalanceOverflow,
}

fn transfer(from: &Account, to: &Account, amount: u64) -> Result<(), TransferError> {
    if from.id == to.id {
        // Volver a bloquear el mismo mutex no está definido como reentrante:
        // la segunda llamada puede bloquearse o hacer panic.
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
    let destination_after = to_balance
        .checked_add(amount)
        .ok_or(TransferError::BalanceOverflow)?;
    *from_balance -= amount;
    *to_balance = destination_after;
    Ok(())
}

let source = Account { id: 1, balance: Mutex::new(10) };
let destination = Account { id: 2, balance: Mutex::new(u64::MAX) };
assert_eq!(transfer(&source, &destination, 1), Err(TransferError::BalanceOverflow));
assert_eq!(*source.balance.lock().unwrap(), 10);
assert_eq!(*destination.balance.lock().unwrap(), u64::MAX);
