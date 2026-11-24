//! ex06 - Enums and `match`.
//!
//! Read `tasks/ex06.md` first. Replace each `todo!()` with an implementation.

/// The data carried by a simplified instruction.
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

/// Return the instruction kind.
pub fn kind(ix: &Instruction) -> &'static str {
    match ix {
        Instruction::Transfer { .. } => "transfer",
        Instruction::CreateAccount { .. } => "create",
        Instruction::Close(_) => "close",
    }
}

/// Return how many lamports the instruction moves.
pub fn lamports_moved(ix: &Instruction) -> u64 {
    match ix {
        Instruction::Transfer { lamports, .. } => *lamports,
        Instruction::CreateAccount { .. } => 0,
        Instruction::Close(_) => 0,
    }
}

/// Return the main account touched by the instruction.
pub fn account_touched(ix: &Instruction) -> &str {
    match ix {
        Instruction::Transfer { from, .. } => from,
        Instruction::CreateAccount { address } => address,
        Instruction::Close(address) => address,
    }
}
