use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize
    };
    // 数列DP
    // 長さと総和のDPで解くパターン
    // dp[i][j] 長さiで総和jになる組み合わせの数
    let p: i64 = 998244353;
    
    let mut dp = vec![vec![0; k+1]; n+1];
    for i in 1..=m {
        dp[1][i] = 1;
    }
    for i in 2..=n {
        for j in i..=k {
            let low = if j <= m {
                1
            } else {
                j-m
            };
            let high = if low+m > j {
                j
            } else {
                low+m
            };
            for l in low..high {
                dp[i][j] += dp[i-1][l];
                dp[i][j] %= p;
            }
        }
    }
    let mut ans = 0;
    for i in 1..=k {
        ans += dp[n][i];
        ans %= p;
    }
    println!("{}", ans);
}
