use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        s: usize,
        abn: [(usize, usize); n]
    };
    let mut dp = vec![false; 10001];
    let mut path = vec!["".to_string(); 10001];
    dp[0] = true;
    for i in 0..n {
        for j in (0..s).rev() {
            if dp[j] {
                dp[j + abn[i].0] = true;
                path[j + abn[i].0] = path[j].to_string() + "H";
                dp[j + abn[i].1] = true;
                path[j + abn[i].1] = path[j].to_string() + "T";
            }
            dp[j] = false;
        }
    }
    if dp[s] {
        println!("{}", "Yes");
        println!("{}", path[s]);
    } else {
        println!("{}", "No");
    }
}
