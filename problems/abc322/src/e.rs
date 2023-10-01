use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        p: usize
    };
    let mut cnak = vec![];
    for i in 0..n {
        input! {
            c: i32,
            ak: [usize; k]
        };
        cnak.push((c, ak));
    }
    // dp[i][j][k][l] = i番目までにj個使ってパラメータkをlにしたときの最小コスト
    let mut dp = vec![vec![vec![vec![std::i32::MAX/2; 26]; k+1]; n+1]; n+1];
    for i in 0..k {
        dp[0][0][i][0] = 0;
    }
    for i in 0..n {
        for j in 0..n {
            for kk in 0..k {
                for l in 0..26 {
                    if l >= cnak[j].1[kk] {
                        dp[i+1][j+1][kk][l] = dp[i+1][j][kk][l].min(dp[i][j][kk][l-cnak[j].1[kk]]+cnak[j].0);
                    }
                    dp[i+1][j][kk][l] = dp[i][j][kk][l].min(dp[i+1][j][kk][l]);
                }
            }
        }
    }
    let mut min = -1;
    println!("{:?}", dp);
    for i in 1..=n {
        let mut local_min = -1;
        for j in 1..=k {
            for l in p..26 {
                if local_min == -1 || local_min > dp[n][i][j][l] {
                    local_min = dp[n][i][j][l];
                }
            }
        }
        if local_min == -1 {
            continue;
        } else {
            if min == -1 || local_min < min {
                min = local_min;
            }
        }
    }
    println!("{}", min);
}