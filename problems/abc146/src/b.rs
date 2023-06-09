use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: u8,
        s: Chars
    };
    println!("{}", s.into_iter().map(|c| (65 + (((c as u8) - 65 + n) % 26)) as char).collect::<String>());
}