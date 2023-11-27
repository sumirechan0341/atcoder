use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        un: [usize; n]
    };
    // dp[i][j][l] = 先頭からi個まででj個使ったときのl(mod d)な値の最大値
    let mut dp = vec![vec![vec![-1; d]; k + 1]; n + 1];
    dp[0][0][0] = 0;
    for i in 0..n {
        for j in 0..=i.min(k) {
            for l in 0..d {
                if dp[i][j][(l + d - (un[i] % d)) % d] != -1 && j < k {
                    // 使った場合
                    dp[i + 1][j + 1][l] =
                        dp[i + 1][j + 1][l].max(dp[i][j][(l + d - (un[i] % d)) % d] + un[i] as i64);
                }
                // 使わなかった場合
                dp[i + 1][j][l] = dp[i + 1][j][l].max(dp[i][j][l]);
            }
        }
    }
    println!("{}", dp[n][k][0]);
}
