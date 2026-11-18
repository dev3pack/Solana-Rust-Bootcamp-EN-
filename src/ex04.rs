//! ex04 - Borrowing.

pub fn total(amounts: &[u64]) -> u64 {
    amounts.iter().sum()
}

pub fn add_fee(amount: &mut u64, fee: u64) {
    *amount += fee; // Dereference with * to change the actual value
}

pub fn count_writable(flags: &[bool]) -> usize {
    flags.iter().filter(|&&flag| flag).count()
}

pub fn append_note(history: &mut Vec<String>, note: &str) {
    history.push(note.to_string());
}