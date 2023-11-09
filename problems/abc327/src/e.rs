use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        pn: [i64; n]
    };
    // dp[i][j] = 先頭i個からj個使ったときの最大スコア
    let mut dp = vec![vec![0.0f64; n+1]; n+1];

    for i in 0..n {
        for j in 0..=i {
            dp[i+1][j] = dp[i+1][j].max(dp[i][j]);
            dp[i+1][j+1] = dp[i+1][j+1].max(dp[i][j]*0.9 + pn[i] as f64);
        }
    }
    
    let mut max = -1200.0;
    for k in 1..=n {
        if max < dp[n][k]/f(k) - 1200.0/(k as f64).sqrt() {
            max = dp[n][k]/f(k) - 1200.0/(k as f64).sqrt();
        }
    }
    println!("{}", max);

}

fn f(k: usize) -> f64 {
    return 10.0*(1.0-0.9f64.powf(k as f64));
}