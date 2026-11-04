//! ex07 - `Result` and the `?` operator.

/// Everything that can go wrong in a transfer.
#[derive(Debug, PartialEq)]
pub enum TransferError {
    ZeroAmount,
    SameAccount,
    InsufficientFunds { available: u64, required: u64 },
}

/// Check the rules that do not depend on the balance.
pub fn validate(from: &str, to: &str, amount: u64) -> Result<(), TransferError> {
    if amount == 0 {
        return Err(TransferError::ZeroAmount);
    }

    if from == to {
        return Err(TransferError::SameAccount);
    }

    Ok(())
}

/// Validate a transfer and return the sender's remaining balance.
pub fn transfer(
    balance: u64,
    from: &str,
    to: &str,
    amount: u64,
) -> Result<u64, TransferError> {
    validate(from, to, amount)?;

    if amount > balance {
        return Err(TransferError::InsufficientFunds {
            available: balance,
            required: amount,
        });
    }

    Ok(balance - amount)
}

/// Pay through `transfer` and propagate any error with `?`.
pub fn pay(balance: u64, from: &str, to: &str, amount: u64) -> Result<u64, TransferError> {
    let remaining_balance = transfer(balance, from, to, amount)?;

    Ok(remaining_balance)
}