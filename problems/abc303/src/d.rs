use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        x: i64,
        y: i64,
        z: i64,
        s: Chars
    };
    // dp[i][j] = 状態jにおけるi番目の最小コスト
    let mut dp = vec![vec![std::i64::MAX/2; 2]; s.len()+1];
    dp[0][0] = 0;
    for i in 0..s.len() {
        dp[i+1][0] = dp[i+1][0].min(dp[i][0] + if s[i] == 'a' { x } else { y }).min(dp[i][1] + z + if s[i] == 'a' { x } else { y });
        dp[i+1][1] = dp[i+1][1].min(dp[i][1] + if s[i] == 'A' { x } else { y }).min(dp[i][0] + z + if s[i] == 'A' { x } else { y });
    }
    println!("{}", dp[s.len()][0].min(dp[s.len()][1]));
}