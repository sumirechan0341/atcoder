use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars,
        t: Chars
    };
    let mut dp = vec![vec![0; t.len()+1]; s.len()+1];
    for i in 0..s.len() {
        for j in 0..t.len() {
            if s[i] == t[j] {
                dp[i+1][j+1] = dp[i+1][j+1].max(dp[i][j]+1);
            }
            dp[i+1][j+1] = dp[i+1][j+1].max(dp[i+1][j]).max(dp[i][j+1]);
        }
    }
    let mut ans = "".to_string();
    let mut i = s.len();
    let mut j = t.len();
    while i > 0 && j > 0 {
        if dp[i][j] != dp[i-1][j-1] {
            ans += &s[i-1].to_string();
            i -= 1;
            j -= 1;
        } else if dp[i][j] != dp[i-1][j] {
            ans += &s[i-1].to_string();
            i -= 1;
        } else if dp[i][j] != dp[i][j-1] {
            ans += &t[j-1].to_string();
            j -= 1;
        } else {
            i -= 1;
            j -= 1;
        }
    }
    for i in (0..ans.len()).rev() {
        print!("{}", ans.chars().nth(i).unwrap());
    }
    println!("{}", "");
}