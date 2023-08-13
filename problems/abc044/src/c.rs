use std::collections::VecDeque;
 
use proconio::{input, marker::Chars};
type VS = Vec<String>;
 
pub fn main() {
    input!{
        n: usize,
        a: usize,
        mut xn: [usize; n]
    };
    // 部分点だけ
    // xn.sort();
    // let mut ans: i64 = 0;
    // for i in 1..(1 as i64)<<n {
    //     let mut ii = i;
    //     let mut local = 0;
    //     let mut count = 0;
    //     for j in 0..n {
    //         if ii & 1 == 1 {
    //             local += xn[j];
    //             count += 1;
    //         }
    //         ii = ii >> 1;
    //     }
    //     if a * count == local {
    //         ans += 1;
    //     }
    // }
    // println!("{}", ans);
    // 解説AC
    /*
    dp[i][j][k] := [0, i]からj個使ってkを作る
     */
    let mut ans = 0;
    // let max = xn.iter().max().unwrap().max(&a);
    let mut dp = vec![vec![vec![0_i64; n*a+1]; n+1]; n+1];
    dp[0][0][0] = 1;
    for i in 0..n+1 {
        for j in 0..n+1 {
            for k in 0..n*a+1 {
                if i >= 1 && xn[i-1] > k {
                    dp[i][j][k] = dp[i-1][j][k];
                } else if i >= 1 && j >= 1 {
                    dp[i][j][k] = dp[i-1][j][k] + dp[i-1][j-1][k-xn[i-1]];
                }
            }
        }
    }
    for i in 1..=n {
        ans += dp[n][i][i*a];
    }
    println!("{}", ans);
}