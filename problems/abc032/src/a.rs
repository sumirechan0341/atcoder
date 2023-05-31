use proconio::{input, marker::Chars};
use num::Integer;

pub fn main() {
    input! {
        a: i32,
        b: i32,
        n: i32
    };
    let m = Integer::lcm(&a, &b);
    let mut ans = 0;
    for _k in 0..(n/m + 2) {
        if ans >= n {
            println!("{}", ans);
            return;
        }
        ans += m;
    }
}

