use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a9: [i64; 9],
        b8: [i64; 8]
    };
    println!(
        "{}",
        a9.iter().sum::<i64>() + 1 - b8.iter().sum::<i64>().max(0)
    );
}
