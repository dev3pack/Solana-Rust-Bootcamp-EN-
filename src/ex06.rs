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

pub fn kind(ix: &Instruction) -> &'static str {
    match ix {
        Instruction::Transfer { .. } => "transfer",
        Instruction::CreateAccount { .. } => "create",
        Instruction::Close(_) => "close",
    }
}

pub fn lamports_moved(ix: &Instruction) -> u64 {
    match ix {
        Instruction::Transfer { lamports, .. } => *lamports,
        Instruction::CreateAccount { .. } => 0,
        Instruction::Close(_) => 0,
    }
}

pub fn account_touched(ix: &Instruction) -> &str {
    match ix {
        Instruction::Transfer { from, .. } => from,
        Instruction::CreateAccount { address } => address,
        Instruction::Close(address) => address,
    }
}