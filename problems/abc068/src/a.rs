use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: String
    };
    println!("{}", "ABC".to_string() + &n);
}