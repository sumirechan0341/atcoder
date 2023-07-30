use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize
    };
    let mut dp = vec![n; n+1];
    dp[0] = 0;
    dp[1] = 1;
    for i in 0..n {
        if dp[i] + 1 < dp[i+1] {
            dp[i+1] = dp[i] + 1;
        }
        for j in 1.. {
            if i + 6_i32.pow(j) as usize > n {
                break;
            }
            if dp[i] + 1 < dp[i+6_i32.pow(j) as usize] {
                dp[i+6_i32.pow(j) as usize] = dp[i] + 1;
            }
        }
        for j in 1.. {
            if i + 9_i32.pow(j) as usize > n {
                break;
            }
            if dp[i] + 1 < dp[i+9_i32.pow(j) as usize] {
                dp[i+9_i32.pow(j) as usize] = dp[i] + 1;
            }
        }  
    }
    println!("{}", dp[n]);
}