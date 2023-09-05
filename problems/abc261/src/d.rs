use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        xn: [i64; n],
        cym: [(usize, i64); m]
    };
    let mut s = vec![0; n+1];
    for i in 0..n {
        s[i+1] = s[i] + xn[i];
    }
    let mut bonus = vec![0; m+1];
    for i in 0..m {
        bonus[i+1] = bonus[i] + cym[i].1;
    }
    let mut dp = vec![0; n+1];
    for i in 0..n {
        for j in 0..m {
            if i+(cym[j].0+1) < n+1 {
                dp[i+(cym[j].0+1)] = dp[i+(cym[j].0+1)].max(dp[i] + s[i+(cym[j].0+1)]-s[i+1] + bonus[j+1]);
            }
            dp[i+1] = dp[i+1].max(dp[i]+xn[i]+if cym[j].0 == i+1 { cym[j].1 } else { 0 });
        }
    }
    println!("{}", dp[n]);
}