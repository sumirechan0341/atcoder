use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        t: u32,
        an: [u32; n]
    };
    let mut ans = 0;
    for i in 0..n {
        if i == n-1 {
            ans += t;
            continue;
        }
        if an[i] + t >= an[i + 1] {
            ans += an[i + 1] - an[i];
        } else {
            ans += t;
        }
    }
    println!("{}", ans);
}