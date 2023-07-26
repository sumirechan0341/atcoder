use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        mut hn: [i32; n]
    };
    hn.sort();
    let mut min = std::i32::MAX;
    for i in 0..n-k+1 {
        if min > hn[i+k-1] - hn[i] {
            min = hn[i+k-1] - hn[i];
        }
    }
    println!("{}", min);
}