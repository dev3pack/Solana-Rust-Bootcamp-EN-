//! ex03 - Ownership.
//!
//! Read `tasks/ex03.md` first. Replace each `todo!()` with an implementation.

/// Build a label from an owned owner name.
///
/// `make_label(String::from("alice"))` should return `"account:alice"`.
/// The function takes ownership of the name.
pub fn make_label(owner: String) -> String {
    let mut res = String::from("account:");
    res.push_str(&owner);
    res
}

/// Return an owned copy of a borrowed string.
///
/// The input is borrowed with `&str`, so the caller keeps ownership.
pub fn duplicate(text: &str) -> String {
    text.to_string()
}

/// Take ownership of an account name and return it together with its length.
///
/// The caller cannot use the original `String` after this call.
pub fn take_and_return(account: String) -> (String, usize) {
    let size = account.len();
    (account, size)
}

/// Consume a vector of account names and return how many there were.
pub fn consume_all(accounts: Vec<String>) -> usize {
    let mut size = 0;
    for account in accounts {
        size += 1;
    }
    size
}
