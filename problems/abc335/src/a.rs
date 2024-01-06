use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: String
    };
    println!("{}", s[0..s.len() - 1].to_string() + "4")
}
