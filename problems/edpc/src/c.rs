use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        abcn: [(i32, i32, i32); n]
    };
    let mut dp = vec![vec![0; 3]; n+1];
    for i in 0..n {
        dp[i+1][0] = dp[i][1].max(dp[i][2])+abcn[i].0;
        dp[i+1][1] = dp[i][2].max(dp[i][0])+abcn[i].1;
        dp[i+1][2] = dp[i][0].max(dp[i][1])+abcn[i].2;
    }
    println!("{}", dp[n][0].max(dp[n][1]).max(dp[n][2]));
}