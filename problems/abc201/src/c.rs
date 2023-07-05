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
    
    dp[2][0] = dp[1][0]*count[1];
    dp[2][1] = dp[1][0]*count[0] + dp[1][1]*count[1];
    dp[2][2] = dp[1][1]*count[0];
    
    dp[3][0] = dp[2][0]*count[1];
    dp[3][1] = dp[2][0]*count[0] + dp[2][1]*count[1];
    dp[3][2] = dp[2][1]*count[0] + dp[2][2]*count[1];
    dp[3][3] = dp[2][2]*count[0];
    
    dp[4][0] = dp[3][0]*count[1];
    dp[4][1] = dp[3][0]*count[0] + dp[3][1]*count[1];
    dp[4][2] = dp[3][1]*count[0] + dp[3][2]*count[1];
    dp[4][3] = dp[3][2]*count[0] + dp[3][3]*count[1];
    dp[4][4] = dp[3][3]*count[0];

    println!("{}", dp[4][count[0]]);
}