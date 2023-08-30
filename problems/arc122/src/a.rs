use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i64; n]
    };
    let p = 1000000007;
    let mut dp = vec![0; n];
    for i in (0..n).rev() {

    }
    // メモ化再帰みたいなの必要な気がする
    // 3 + dp[2] + (3-1) + dp[1] = 3 + 2 + 3 -5
}
