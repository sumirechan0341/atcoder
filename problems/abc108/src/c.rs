use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: i64,
        k: i64
    };
    let mut ans = 0;
    // 自明な解
    // a, b, cすべてkの倍数
    // |a, b, cの倍数| ^ 3
    ans += (n / k).pow(3); 

    // a = b = c (mod k)
    // 偶数倍ならすべて kl + k/2 (l = 0..)を割り当てれば作れる
    // |kl + k/2 (l = 0..)| ^ 3
    if k % 2 == 0 {
        let mut base = (n + k/2) / k;
        ans += base.pow(3);
    }
    println!("{}", ans);
}