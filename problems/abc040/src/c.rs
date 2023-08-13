use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i64; n]
    };
    let mut dp = vec![0; n+1];
    dp[0] = 0;
    dp[1] = (an[0]-an[1]).abs();
    for i in 2..n {
        dp[i] = (dp[i-1] + (an[i-1] - an[i]).abs()).min(dp[i-2] + (an[i-2] - an[i]).abs());
    }
    println!("{}", dp[n-1]);
}