use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: String
    };
    println!("{}", s.split(',').collect::<Vec<_>>().join(" "));
}