// fn base_fee(signatures: u64) -> u64 {
//     signatures * 5_000 // base fee
// }

// fn total(amounts: &[u64]) -> u64 {
//     let mut sum = 0;
//     for amount in amounts {
//         sum += amount;
//     }
//     sum
// }

pub fn main() {
    let mut _balance: i32 = 5_000;
    {
        let _balance: i32 = _balance + 1;
        println!("{_balance}");
    }

    print!("{_balance}\n");
}
