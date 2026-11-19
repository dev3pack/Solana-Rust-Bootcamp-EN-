//! ex07 - `Result` and the `?` operator.

#[derive(Debug, PartialEq)]
pub enum TransferError {
    ZeroAmount,
    SameAccount,
    InsufficientFunds { available: u64, required: u64 },
}

pub fn validate(from: &str, to: &str, amount: u64) -> Result<(), TransferError> {
    if amount == 0 {
        return Err(TransferError::ZeroAmount);
    }
    if from == to {
        return Err(TransferError::SameAccount);
    }
    Ok(())
}

pub fn transfer(balance: u64, from: &str, to: &str, amount: u64) -> Result<u64, TransferError> {
    validate(from, to, amount)?; // The ? operator automatically returns early if Err

    if amount > balance {
        return Err(TransferError::InsufficientFunds {
            available: balance,
            required: amount,
        });
    }

    Ok(balance - amount)
}

pub fn pay(balance: u64, from: &str, to: &str, amount: u64) -> Result<u64, TransferError> {
    let remaining = transfer(balance, from, to, amount)?;
    Ok(remaining)
}