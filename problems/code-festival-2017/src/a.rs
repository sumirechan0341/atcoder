use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    println!("{}", s[..s.len()-8].iter().collect::<String>())
}