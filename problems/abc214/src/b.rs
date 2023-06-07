use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: i32,
        t: i32
    };
    let mut ans = 0;
    for a in 0..101 {
        for b in 0..101 {
            for c in 0..101 {
                if a+b+c <= s && a*b*c <= t {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}