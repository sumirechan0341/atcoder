use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: String
    };
    println!("{}", if a == "a" { "-1" } else { "a" });
}