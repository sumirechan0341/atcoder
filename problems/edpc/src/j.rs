use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [f64; n]
    };
    // dp[i][j][k] = 残り3貫の皿がi個、残り2貫の皿がj個、残り1貫の皿がk個のときの期待値
    let mut dp = vec![vec![vec![0.0; n+2]; n+2]; n+2];
    let mut count = vec![0; 4];
    for i in 0..n {
        count[an[i] as usize] += 1;
    }
    // 問題の簡単で小さい部分から構築していくイメージで
    // N = 3
    // dp[0][0][1] = 1 + 2/3*dp[0][0][1] + 1/3*dp[0][0][0]
    // dp[0][1][0] = 1 + 2/3*dp[0][1][0] + 1/3*dp[0][0][1]
    // dp[1][0][0] = 1 + 2/3*dp[1][0][0] + 1/3*dp[0][1][0]

    // dp[i][j][k] = 1 + (N-i-j-k)/N*dp[i][j][k] + k/N*dp[i][j][k-1] + j/N*dp[i][j-1][k+1] + i/N*dp[i-1][j+1][k]
    // (1-(N-i-j-k)/N)dp[i][j][k] = 1 + k/N*dp[i][j][k-1] + j/N*dp[i][j-1][k+1] + i/N*dp[i-1][j+1][k]
    // dp[i][j][k] = N/(i+j+k) * (1 + k/N*dp[i][j][k-1] + j/N*dp[i][j-1][k+1] + i/N*dp[i-1][j+1][k])
    // dp[i][j][k] = 1/(i+j+k) * (N + k*dp[i][j][k-1] + j*dp[i][j-1][k+1] + i*dp[i-1][j+1][k])
    for i in 0..=n {
        for j in 0..=n {
            for k in 0..=n {
                if i == 0 && j == 0 && k == 0 {
                    dp[i][j][k] = 0.0;
                } else {
                    dp[i][j][k] +=  1.0/(i+j+k) as f64*(n as f64);
                    if k != 0 {
                        dp[i][j][k] += 1.0/(i+j+k) as f64 * k as f64*dp[i][j][k-1];
                    }
                    if j != 0 {
                        dp[i][j][k] += 1.0/(i+j+k) as f64 * j as f64*dp[i][j-1][k+1]
                    }
                    if i != 0 {
                        dp[i][j][k] += 1.0/(i+j+k) as f64*(i as f64*dp[i-1][j+1][k]);
                    }
                }
            }
        }
    }
    println!("{}", dp[count[3]][count[2]][count[1]]);
}