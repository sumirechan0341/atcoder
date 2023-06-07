use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut stn: [(String, i32); n]
    };
    stn.sort_by_key(|a| a.1);
    println!("{}", stn[n-2].0);
}