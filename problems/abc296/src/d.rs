use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i64,
        m: i64
    };
    if n.saturating_mul(n) < m {
        println!("{}", -1);
        return;
    }
    let mut i = 1;
    let mut min = std::i64::MAX/2;
    while i*i < m {
        i+=1;
    }
    for a in 1..=i {
        let b = (m+a-1)/a;
        if b <= n && min > a*b {
            min = a*b;
        }
    }
    if min == std::i64::MAX/2 {
        println!("{}", -1);
    } else {
        println!("{}", min);
    }
}