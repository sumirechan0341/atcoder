use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        xn: [i64; n],
        cym: [(usize, i64); m]
    };
    // dp[i][j] = i番目の最大値。ただし連続している回数はj回である。
    let mut bonus = vec![0; n+1];
    for &(c, y) in &cym {
        bonus[c] = y;
    }
    let mut dp = vec![vec![0; n+1]; n+1];
    for i in 0..n {
        for j in 0..n {
            if j > i {
                break;
            }
            // 表
            dp[i+1][j+1] = dp[i+1][j+1].max(dp[i][j] + xn[i] + bonus[j+1]);
            // 裏
            dp[i+1][0] = dp[i+1][0].max(dp[i][j])
        }
    }
    println!("{}", dp[n].iter().max().unwrap());
}