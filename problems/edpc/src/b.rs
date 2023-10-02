use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        hn: [i32; n]
    };
    let mut dp = vec![std::i32::MAX/2; n];
    dp[0] = 0;
    for i in 0..n-1 {
        for j in 1..=k {
            if i+j >= n {
                break;
            }
            dp[i+j] = dp[i+j].min(dp[i]+(hn[i]-hn[i+j]).abs());
        }
    }
    println!("{}", dp[n-1]);
}