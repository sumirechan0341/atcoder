use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        mut ln: [i32; n]
    };
    ln.sort();
    println!("{}", ln[n-k..].into_iter().sum::<i32>());
}