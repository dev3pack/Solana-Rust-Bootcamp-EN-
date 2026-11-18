//! ex05 - Structs and `impl`.

#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub address: String,
    pub lamports: u64,
    pub owner: String,
}

impl Account {
    pub fn new(address: String) -> Self {
        Self {
            address,
            lamports: 0,
            owner: String::from("system"),
        }
    }

    pub fn deposit(&mut self, amount: u64) {
        self.lamports += amount;
    }

    pub fn withdraw(&mut self, amount: u64) -> bool {
        if self.lamports >= amount {
            self.lamports -= amount;
            true
        } else {
            false
        }
    }

    pub fn balance_in_sol(&self) -> u64 {
        self.lamports / 1_000_000_000
    }
}