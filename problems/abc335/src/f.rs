use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n]
    };
    let mut dp = vec![0; n];
    dp[0] = 1;
    for i in 0..n {
        for j in (i + 1..n).step_by(an[i]) {
            dp[j] += dp[i];
            dp[j] %= 998244353;
        }
    }
    let mut ans = 0;
    for i in 0..n {
        ans += dp[i];
        ans %= 998244353;
    }
    println!("{:?}", dp);
    println!("{}", ans);
}
