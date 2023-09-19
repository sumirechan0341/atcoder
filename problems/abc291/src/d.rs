use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        abn: [(i32, i32); n],
    };
    let p = 998244353;
    let mut dp = vec![vec![0; 2]; n];
    dp[0][0] = 1;
    dp[0][1] = 1;
    // dp[i][j] := i番目までの並べ方の総数、ただしi番目の状態はj (j = 0 ならば表, j = 1 ならば裏)
    for i in 0..n-1 {
        if abn[i+1].0 != abn[i].0 {
            dp[i+1][0] += dp[i][0];
            dp[i+1][0] %= p;
        }
        if abn[i+1].0 != abn[i].1 {
            dp[i+1][0] += dp[i][1];
            dp[i+1][0] %= p;
        }
        if abn[i+1].1 != abn[i].0 {
            dp[i+1][1] += dp[i][0];
            dp[i+1][1] %= p;
        }
        if abn[i+1].1 != abn[i].1 {
            dp[i+1][1] += dp[i][1];
            dp[i+1][1] %= p;
        }
    }
    println!("{}", (dp[n-1][0] + dp[n-1][1])%p);
}