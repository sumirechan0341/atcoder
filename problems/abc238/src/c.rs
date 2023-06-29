use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: u64
    };
    let mut count = 0;
    let p = 998244353;
    let p2inv = 499122177;
    let mut digit = 0;
    while n >= 10_u64.pow(digit) {
        let d = 10_u64.pow(digit);
        if n / d < 10 {
            count += ((((n-d+1) % p) * ((n-d+2) % p) % p) * p2inv) % p;
        } else {
            count += ((((1+10_u64.pow(digit)*9) % p) * (10_u64.pow(digit)*9 % p) % p) * p2inv) % p;
        }
        count %= p;
        digit += 1;
    }
    println!("{}", count);
}