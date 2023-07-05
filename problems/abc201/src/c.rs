use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: Chars
    };
    let mut dp = vec![vec![0; 5]; 5];
    let mut count = vec![0; 3];
    for i in 0..10 {
        match s[i] {
            'o' => {
                count[0] += 1;
            },
            '?' => {
                count[1] += 1;
            },
            _ => {

            }
        }
    }
    if count[0] > 4 {
        println!("{}", 0);
        return;
    }
    dp[1][0] = count[1];
    dp[1][1] = count[0];
    // dp[i][j] i桁でj個oを使用
    for i in 2..=4 {
        for j in 0..=i {
            if j == 0 {
                dp[i][j] = dp[i-1][j]*count[1];
            } else if j == i {
                dp[i][j] = dp[i-1][j-1]*count[0];
            } else {
                dp[i][j] = dp[i-1][j-1]*count[0] + dp[i-1][j]*count[1];
            }
            
        }
    }
    println!("{}", dp[4][3]);
    println!("{}", dp[4][4]);
    println!("{}", dp[4][count[0]..=4].iter().sum::<usize>());
}