//! ex05 - Structs and impl.

#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub address: String,
    pub lamports: u64,
    pub owner: String,
}

impl Account {
    /// Create a new account with zero lamports and system ownership.
    pub fn new(address: String) -> Self {
        Self {
            address,
            lamports: 0,
            owner: String::from("system"),
        }
    }

    /// Add lamports to the account.
    pub fn deposit(&mut self, amount: u64) {
        self.lamports += amount;
    }

    /// Withdraw lamports if sufficient balance is available.
    pub fn withdraw(&mut self, amount: u64) -> bool {
        if amount > self.lamports {
            return false;
        }

        self.lamports -= amount;
        true
    }

    /// Return the balance in whole SOL.
    pub fn balance_in_sol(&self) -> u64 {
        self.lamports / 1_000_000_000
    }
}
