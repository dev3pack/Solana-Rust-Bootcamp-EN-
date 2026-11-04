//! ex08 - Capstone: a mini bank account.
//!
//! Read `tasks/ex08.md` first. Replace each `todo!()` with an implementation.
//! No `HashMap` and no traits: only a struct, a `Vec<String>` history, and
//! `Result`.

pub const LAMPORTS_PER_SOL :u64 = 1_000_000_000;

/// Everything that can go wrong in the mini bank.
#[derive(Debug, PartialEq)]
pub enum BankError {
    ZeroAmount,
    InsufficientFunds { available: u64, required: u64 },
}

/// A bank account with a human-readable history.
#[derive(Debug, Clone, PartialEq)]
pub struct BankAccount {
    pub owner: String,
    pub lamports: u64,
    pub history: Vec<String>,
}

impl BankAccount {
    /// Create an account with zero lamports and an empty history.
    pub fn new(owner: &str) -> Self {
        Self{
             owner : owner.to_string(),
            lamports : 0,
            history : Vec::new(),
        }
    }

    /// Deposit lamports and record it in the history.
    ///
    /// Return `BankError::ZeroAmount` for a zero deposit. A successful deposit
    /// appends `"deposit <amount>"` to the history.
    pub fn deposit(&mut self, amount: u64) -> Result<(), BankError> {
        if amount == 0 {
            Err(BankError::ZeroAmount)
        }else{
            self.lamports += amount;
            self.history.push(format!("deposit {}", amount));
            Ok(())
        }
    }

    /// Withdraw lamports and record it in the history.
    ///
    /// Return `BankError::ZeroAmount` for a zero withdrawal. Return
    /// `BankError::InsufficientFunds` when the balance is too low. A successful
    /// withdrawal appends `"withdraw <amount>"` to the history.
    pub fn withdraw(&mut self, amount: u64) -> Result<(), BankError> {
        if amount == 0 {
            Err(BankError::ZeroAmount)
        }else{
            if self.lamports < amount{
                Err(BankError::InsufficientFunds { available: self.lamports, required: amount })
            }else {
                self.lamports -= amount;
                self.history.push(format!("withdraw {}", amount));
                Ok(())
        }
        }
    }

    /// Return the balance in whole SOL, dropping any remainder.
    pub fn balance_in_sol(&self) -> u64 {
        let balance_sol = self.lamports / LAMPORTS_PER_SOL;
        balance_sol
    }
}
