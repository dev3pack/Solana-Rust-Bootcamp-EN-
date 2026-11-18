//! ex02 - Functions and control flow.

pub fn base_fee(signatures: u64) -> u64 {
    signatures * 5_000
}

pub fn total(amounts: &[u64]) -> u64 {
    amounts.iter().sum()
}

pub fn count_above(amounts: &[u64], threshold: u64) -> usize {
    amounts.iter().filter(|&&amount| amount > threshold).count()
}

pub fn clamp_transfer(amount: u64, balance: u64) -> u64 {
    if amount > balance {
        balance
    } else {
        amount
    }
}