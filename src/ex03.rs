//! ex03 - Ownership.
//!
//! Read `tasks/ex03.md` first. Replace each `todo!()` with an implementation.

/// Build a label from an owned owner name.
///
/// `make_label(String::from("alice"))` should return `"account:alice"`.
/// The function takes ownership of the name.
pub fn make_label(owner: String) -> String {
    format!("account:{}", owner)
    // uses format instead of print or println to return as string, note the absence of ; at the end.
}

/// Return an owned copy of a borrowed string.
///
/// The input is borrowed with `&str`, so the caller keeps ownership.
pub fn duplicate(text: &str) -> String {
    let copy: String = text.to_string();
    copy
    // returns the copy that now actually owns the data that was borrowed
}

/// Take ownership of an account name and return it together with its length.
///
/// The caller cannot use the original `String` after this call.
pub fn take_and_return(account: String) -> (String, usize) {
    let taken_account: String = account;
    let len: usize = taken_account.len();
    (taken_account, len)
}

/// Consume a vector of account names and return how many there were.
pub fn consume_all(accounts: Vec<String>) -> usize {
    accounts.len()
}
