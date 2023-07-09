use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        l: usize
    };
    // todo メモ化再帰 トップダウン解法でも解いてみたい
    let mut dp: Vec<Vec<i64>> = vec![vec![0; 13]; (l+1) as usize];
    for i in 1..l+1 {
        for j in 1..13 {
            if j == 1 {
                dp[i][j] = 1;
            } else if i == j {
                dp[i][j] = 1;
            } else if i < j {
                dp[i][j] = 0;
            } else {
                for k in 0..i-j+1 {
                    dp[i][j] += dp[i-1-k][j-1];
                }
            }
        }
    }
    println!("{}", dp[l][12]);
}