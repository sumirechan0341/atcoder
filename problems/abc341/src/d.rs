use num::integer::lcm;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize
    };
    let l = lcm(n, m);
    let mut low = 0;
    let mut high = 10_usize.pow(18);
    while high - low > 1 {
        let mid = (low + high) / 2;
        let mut cnt = mid / n + mid / m;
        cnt -= mid / l * 2;
        if cnt >= k {
            high = mid;
        } else {
            low = mid;
        }
    }
    println!("{}", low + 1);
}
