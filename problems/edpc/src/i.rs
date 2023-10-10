use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        pn: [f64; n]
    };
    // dp[i][j] = i回投げてj回表が出る確率
    let mut dp = vec![vec![0.0; n+1]; n+1];
    dp[0][0] = 1.0;
    for i in 0..n {
        for j in 0..=i {
            dp[i+1][j+1] += dp[i][j]*pn[i];
            dp[i+1][j] += dp[i][j]*(1.0-pn[i]);
        }
    }
    let mut ans = 0.0;
    for i in (n+1)/2..=n {
        ans += dp[n][i];
    }
    println!("{}", ans);
}