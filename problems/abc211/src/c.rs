use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: Chars
    };
    let chokudai = vec!['c', 'h', 'o', 'k', 'u', 'd', 'a', 'i'];
    let p = 1000000007;
    let mut dp: Vec<Vec<i64>> = vec![vec![0; s.len()+1]; 9];
    
    for i in 0..=s.len() {
        dp[0][i] = 1;
    }
    for i in 1..=8 {
        dp[i][0] = 0;
    }
    for i in 1..=8 {
        for j in 1..=s.len() {
            if s[j-1] != chokudai[i-1] {
                dp[i][j] = dp[i][j-1];
            } else {
                dp[i][j] = dp[i][j-1] + dp[i-1][j-1];
                dp[i][j] %= p;
            }
        }
    }
    println!("{}", dp[8][s.len()]);
}