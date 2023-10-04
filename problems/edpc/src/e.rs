use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        w: usize,
        wvn: [(usize, usize); n]
    };
    // dp[i][j] ... i番目まで使って価値jにしたときの最小重さ
    let mut dp = vec![vec![std::usize::MAX/2; 100001]; n+1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..100000 {
            if wvn[i].1+j <= 100000 {
                dp[i+1][wvn[i].1+j] = dp[i+1][wvn[i].1+j].min(dp[i][j] + wvn[i].0);
            }
            dp[i+1][j] = dp[i+1][j].min(dp[i][j]);
        }
    }
    let mut max = 0;
    for i in 0..=n {
        for j in 0..100001 {
            if dp[i][j] <= w && j > max {
                max = j;
            }
        }
    }
    println!("{}", max);
}