use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        w: usize,
        wvn: [(usize, usize); n]
    };
    let mut dp = vec![vec![0; n+1]; w+1];
    for i in 0..=w {
        for j in 0..n {
            if wvn[j].0+i <= w {
                dp[wvn[j].0+i][j+1] = dp[wvn[j].0+i][j+1].max(dp[i][j] + wvn[j].1);
            }
            dp[i][j+1] = dp[i][j].max(dp[i][j+1]);
        }
    }
    println!("{}", dp[w].iter().max().unwrap());
}