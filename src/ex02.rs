
/// The base fee for a transaction, in lamports.
pub fn base_fee(signatures: u64) -> u64 {
    signatures * 5_000
}

/// Sum every amount in the slice.
pub fn total(amounts: &[u64]) -> u64 {
    let mut sum = 0;

    for amount in amounts {
        sum += amount;
    }

    sum
}

/// Count amounts strictly greater than threshold.
pub fn count_above(amounts: &[u64], threshold: u64) -> usize {
    let mut count = 0;

    for amount in amounts {
        if *amount > threshold {
            count += 1;
        }
    }

    count
}

/// Never transfer more than the balance.
pub fn clamp_transfer(amount: u64, balance: u64) -> u64 {
    if amount > balance {
        balance
    } else {
        amount
    }
}
