use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i64; n]
    };
    let p = 1000000007;
    let mut dp = vec![0; n];
    dp[0] = an[n-1];
    dp[1] = 2*an[n-2];
    for i in 2..n {
        dp[i] = an[n-i-1]+dp[i-1] + (an[n-i-1]-an[n-i])+dp[i-2];
    }
    println!("{:?}", dp);
    // メモ化再帰みたいなの必要な気がする
    // 3 + dp[2] + (3-1) + dp[1] = 3 + 2 + 3 -5
}