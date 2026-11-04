
/// Build a label from an owned owner name.
pub fn make_label(owner: String) -> String {
    format!("account:{}", owner)
}

/// Return an owned copy of a borrowed string.
pub fn duplicate(text: &str) -> String {
    text.to_string()
}

/// Take ownership of an account name and return it together with its length.
pub fn take_and_return(account: String) -> (String, usize) {
    let length = account.len();
    (account, length)
}

/// Consume a vector of account names and return how many there were.
pub fn consume_all(accounts: Vec<String>) -> usize {
    accounts.len()
}
