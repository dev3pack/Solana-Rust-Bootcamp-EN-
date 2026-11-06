//! ex05 - Structs and `impl`.
//!
//! Read `tasks/ex05.md` first. Replace each `todo!()` with an implementation.

/// A simplified Solana account.
#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub address: String,
    pub lamports: u64,
    pub owner: String,
}

impl Account {
    /// Create an account with zero lamports.
    ///
    /// The owner of a new account is the `"system"` program.
    pub fn new(address: String) -> Self {
        Self {
            address,
            lamports: 0,
            owner: "system".to_string(),
        }
    }

    /// Add lamports to the account.
    pub fn deposit(&mut self, amount: u64) {
        self.lamports = self.lamports.saturating_add(amount);
    }

    /// Remove lamports from the account.
    ///
    /// Return `true` when the withdrawal succeeded. Return `false` and leave
    /// the balance unchanged when there are not enough lamports.
    pub fn withdraw(&mut self, amount: u64) -> bool {
        if self.lamports >= amount {
            self.lamports = self.lamports.saturating_sub(amount);
            true
        } else {
            false
        }
    }

    /// Return the balance in whole SOL, dropping any remainder.
    ///
    /// This is for display only. Never use a floating-point number for money.
    pub fn balance_in_sol(&self) -> u64 {
        self.lamports / 1_000_000_000
    }
}
