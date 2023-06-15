use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        w: Chars
    };
    println!("{}", if w.into_iter().fold(0, |acc, c| acc ^ c as u8) == 0 { "Yes" } else { "No" });
}