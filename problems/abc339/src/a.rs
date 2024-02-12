use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: String
    };
    println!("{}", s.split('.').nth(s.split('.').count() - 1).unwrap());
}
