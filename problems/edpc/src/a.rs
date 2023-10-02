use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        hn: [i32; n]
    };
    let mut dp = vec![std::i32::MAX/2; n+1];
    dp[0] = 0;
    for i in 0..n-1 {
        dp[i+1] = dp[i+1].min(dp[i] + (hn[i]-hn[i+1]).abs());
        if i != n-2 {
            dp[i+2] = dp[i+2].min(dp[i] + (hn[i]-hn[i+2]).abs());
        }
    }
    println!("{}", dp[n-1]);
}