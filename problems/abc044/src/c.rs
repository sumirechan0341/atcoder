use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        a: i64,
        xn: [i64; n]
    };
    // dp[i][j] := i番目以降の数だけ使ってjが作れる数
    let mut dp = vec![vec![0; n]; (a+1) as usize];

    let mut ans: i64 = 0;
    for i in 1..(1 as i64)<<n {
        let mut ii = i;
        let mut local = 0;
        let mut count = 0;
        for j in 0..n {
            if ii & 1 == 1 {
                local += xn[j];
                count += 1;
            }
            ii = ii >> 1;
        }
        if a * count == local {
            ans += 1;
        }
    }
    println!("{}", ans);
}