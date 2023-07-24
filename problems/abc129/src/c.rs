use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        am: [usize; m]
    };
    let p = 1000000007;
    let mut ok = vec![true; n+1];
    for i in 0..m {
        ok[am[i]] = false;
    }
    let mut dp = vec![0; n+1];
    dp[0] = 1;
    dp[1] = if ok[1] { 1 } else { 0 };
    for i in 2..=n {
        if ok[i] {
            dp[i] += (dp[i-1] % p) + (dp[i-2] % p);
            dp[i] %= p;
        }
    }
    println!("{}", dp[n]);
}