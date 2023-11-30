use proconio::{input, marker::Chars};
pub fn main() {
    input! {
        n: usize,
        m: usize,
        an: [i64; n]
    };
    // dp[i][j] = i番目まででj個使ったときの最大値
    let mut dp = vec![vec![std::i64::MIN / 2; m + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..m.min(i + 1) {
            dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j] + (j as i64 + 1) * an[i]);
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
        }
    }
    let mut max = std::i64::MIN;
    for i in m..=n {
        if max < dp[i][m] {
            max = dp[i][m];
        }
    }
    println!("{}", max);
}
