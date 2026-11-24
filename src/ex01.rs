//! ex01 - Variables and types.
//!
//! Read `tasks/ex01.md` first. Replace each `todo!()` with an implementation.

/// The number of lamports in one SOL.
pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

/// Convert whole SOL into lamports.
pub fn sol_to_lamports(sol: u64) -> u64 {
    sol * LAMPORTS_PER_SOL
}

/// Convert lamports into whole SOL, dropping any remainder.
pub fn lamports_to_sol(lamports: u64) -> u64 {
    lamports / LAMPORTS_PER_SOL
}

/// Add a bonus to a balance and return the new balance.
pub fn apply_bonus(balance: u64, bonus: u64) -> u64 {
    balance + bonus
}

/// Double a value.
///
/// Use shadowing in your implementation.
pub fn double(value: u64) -> u64 {
    let value = value * 2;
    value
}

/// Return `true` when the balance is zero.
pub fn is_empty(balance: u64) -> bool {
    balance == 0
}
