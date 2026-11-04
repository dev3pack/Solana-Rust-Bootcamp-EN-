
/// A simplified Solana account.
#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub address: String,
    pub lamports: u64,
    pub owner: String,
}

impl Account {
    /// Create an account with zero lamports.
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

    /// Remove lamports from the account.
    pub fn withdraw(&mut self, amount: u64) -> bool {
        if self.lamports >= amount {
            self.lamports -= amount;
            true
        } else {
            false
        }
    }

    /// Return the balance in whole SOL.
    pub fn balance_in_sol(&self) -> u64 {
        self.lamports / crate::ex01::LAMPORTS_PER_SOL
    }
}
