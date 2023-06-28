use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize
    };
    let p = 998244353;
    let mut dp = vec![vec![0; 10]; n+1];
    for i in 1..=9 {
        dp[1][i] = 1;
    }
    for i in 2..=n {
        dp[i][1] += dp[i-1][1] + dp[i-1][2];
        dp[i][1] %= p;
        for j in 2..=8 {
            dp[i][j] += (dp[i-1][j-1] + dp[i-1][j]) % p + dp[i-1][j+1];
            dp[i][j] %= p;
        }
        dp[i][9] += dp[i-1][8] + dp[i-1][9];
        dp[i][9] %= p;
    }
    println!("{}", dp[n].iter().fold(0, |acc, x| (acc+x)%p));
}