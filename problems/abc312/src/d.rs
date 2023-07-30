use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: Chars
    };
    let p = 998244353;
    let mut dp = vec![vec![0; s.len()+1]; s.len()+1];
    dp[0][0] = 1;
    // dp[i][j] i番目までで左カッコがj個余っている
    for i in 0..s.len() {
        if s[i] == '(' {
            for j in 0..=i {
                dp[i+1][j+1] += dp[i][j];
                dp[i+1][j+1] %= p;
            }
        } else if s[i] == ')' {
            for j in 1..=i {
                dp[i+1][j-1] += dp[i][j];
                dp[i+1][j-1] %= p;
            }
        } else {
            dp[i+1][1] += dp[i][0];
            dp[i+1][1] %= p;
            for j in 1..=i {
                dp[i+1][j+1] += dp[i][j];
                dp[i+1][j+1] %= p;
                dp[i+1][j-1] += dp[i][j];
                dp[i+1][j-1] %= p;
            }
        }
    }
    println!("{}", dp[s.len()][0]);
}