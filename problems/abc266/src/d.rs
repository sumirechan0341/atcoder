use std::collections::HashMap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        txan: [(usize, usize, i64); n]
    };
    let mut dp = vec![vec![-1; 5]; 100001];
    let mut map = HashMap::new();
    for i in 0..n {
        map.insert((txan[i].0, txan[i].1), txan[i].2);
    }
    dp[0][0] = 0;
    for i in 0..100000 {
        for j in 0..5 {
            if dp[i][j] != -1 {
                if j != 4 {
                    dp[i + 1][j + 1] =
                        dp[i + 1][j + 1].max(dp[i][j] + map.get(&(i + 1, j + 1)).unwrap_or(&0));
                }
                if j != 0 {
                    dp[i + 1][j - 1] =
                        dp[i + 1][j - 1].max(dp[i][j] + map.get(&(i + 1, j - 1)).unwrap_or(&0));
                }

                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j] + map.get(&(i + 1, j)).unwrap_or(&0));
            }
        }
    }
    println!("{}", dp[100000].iter().max().unwrap().max(&0));
}
