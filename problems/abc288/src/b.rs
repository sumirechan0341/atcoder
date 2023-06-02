use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: [String; n]
    };
    s[0..k].sort();
    println!("{}", s[0..k].join("\n"));
}