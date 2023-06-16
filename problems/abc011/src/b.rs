use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: String
    };
    println!("{}", s.to_ascii_uppercase()[0..1].to_string() + &s.to_ascii_lowercase()[1..]);
}