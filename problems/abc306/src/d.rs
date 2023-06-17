use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        xyn: [(i32, i64); n]
    };
    let mut dp = vec![vec![0_i64; n+1]; 2];
    for i in 0..n {
        if xyn[i].0 == 0 {
            // 解毒
            if xyn[i].1 > 0 {
                dp[0][i+1] = dp[1][i].max(dp[0][i]) + xyn[i].1;   
            } else {
                dp[0][i+1] = dp[0][i].max(dp[1][i]+xyn[i].1);
                dp[1][i+1] = dp[1][i];
            }
        } else {
            // 毒
            if xyn[i].1 > 0 {
                dp[0][i+1] = dp[0][i];
                dp[1][i+1] = (dp[0][i] + xyn[i].1).max(dp[1][i]);
            } else {
                dp[0][i+1] = dp[0][i];
                dp[1][i+1] = dp[1][i];
            }
            
        }
    }
    println!("{}", dp[0][n].max(dp[1][n]));
}