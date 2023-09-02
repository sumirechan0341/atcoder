use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i64; n]
    };
    let p = 1000000007;
    let mut dp = vec![0; n];
    let mut count = vec![0; n];
    if n == 1 {
        println!("{}", an[0]);
        return;
    }

    count[0] = 1;
    count[1] = 2;
    for i in 2..n {
        count[i] = (count[i-1] + count[i-2])%p;
    }
    dp[0] = an[n-1];
    dp[1] = 2*an[n-2];
    for i in 2..n {
        dp[i] = (((an[n-i-1]*count[i-1])%p+dp[i-1])%p + p + (((an[n-i-1]-an[n-i])*count[i-2])%p+dp[i-2])%p + p)%p;
    }
    println!("{}", dp[n-1]);
  
}