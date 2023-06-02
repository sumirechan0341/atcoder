use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        _n: usize,
        s: String
    };
    println!("{}", s.replace("na", "nya"));
}