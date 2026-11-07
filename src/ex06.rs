//! ex06 - Enums and `match`.
//!
//! Read `tasks/ex06.md` first. Replace each `todo!()` with an implementation.

/// The data carried by a simplified instruction.
///
/// This enum models instruction *data*. The accounts are passed separately by
/// the transaction, just like an Anchor instruction receives a `Context`.
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Transfer {
        from: String,
        to: String,
        lamports: u64,
    },
    CreateAccount {
        address: String,
    },
    Close(String),
}

/// Return the instruction kind: `"transfer"`, `"create"`, or `"close"`.
pub fn kind(ix: &Instruction) -> &'static str {
    match ix {
        Instruction::CreateAccount{ .. } => "create",
        Instruction::Transfer{ .. } => "transfer",
        Instruction::Close{ .. } => "close",
    }
}

/// Return how many lamports the instruction moves.
///
/// Only `Transfer` moves lamports. Every other variant returns `0`.
pub fn lamports_moved(ix: &Instruction) -> u64 {
    match ix {
        Instruction::Close{ .. } => 0,
        Instruction::CreateAccount { .. } => 0,
        Instruction::Transfer { lamports, ..} => *lamports,
    }
}

/// Return the account that is the main subject of the instruction.
///
/// For `Transfer` return the sender. For `CreateAccount` return the new
/// address. For `Close` return the address being closed.
pub fn account_touched(ix: &Instruction) -> &str {
    match ix {
        Instruction::Close(String) => String,
        Instruction::CreateAccount {address} => address,
        Instruction::Transfer { from , ..} => from,
    }
}
