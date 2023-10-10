use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [Chars; h]
    };
    let p = 1000000007;
    let mut dp = vec![vec![0; w]; h];
    dp[0][0] = 1;
    for i in 0..h {
        for j in 0..w {
            if i+1 < h && ahw[i+1][j] == '.' {
                dp[i+1][j] += dp[i][j];
                dp[i+1][j] %= p;
            }
            if j+1 < w && ahw[i][j+1] == '.' {
                dp[i][j+1] += dp[i][j];
                dp[i][j+1] %= p;
            }
        }
    }
    println!("{}", dp[h-1][w-1]);
}