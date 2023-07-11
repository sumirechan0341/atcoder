use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize
    };
    let p = 1000000007;
    let mut dp: Vec<Vec<i64>> = vec![vec![0; 3]; n+1];
    dp[0][0] = 1;
    dp[0][1] = 0;
    dp[0][2] = 0;
    for i in 1..=n {
        dp[i][0] = (dp[i-1][0]*8) % p;
        dp[i][1] = (dp[i-1][0]*2 + dp[i-1][1]*9) % p;
        dp[i][2] = (dp[i-1][1] + dp[i-1][2]*10) % p;
    }
    println!("{}", dp[n][2]);
}