//! ex03 - Ownership.

pub fn make_label(owner: String) -> String {
    format!("account:{}", owner)
}

pub fn duplicate(text: &str) -> String {
    text.to_string()
}

pub fn take_and_return(account: String) -> (String, usize) {
    let len = account.len();
    (account, len)
}

pub fn consume_all(accounts: Vec<String>) -> usize {
    accounts.len()
}