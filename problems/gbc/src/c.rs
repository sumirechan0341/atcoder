use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut an: [i64; n]
    };
    let p = 998244353;
    let mut ans = 1_i64;
    an.sort();
    let mut count = 1;
    let mut prev = an[0];
    for i in 1..n {
        if an[i] == prev {
            count += 1;
        } else {
            prev = an[i];
            ans *= fact_mod(count, p);
            ans %= p;
            count = 1;
        }
    }
    ans *= fact_mod(count, p);
    ans %= p;
    println!("{}", ans);
}

fn fact_mod(n: i64, p: i64) -> i64 {
    let mut res = 1;
    for i in 1..=n {
        res *= i;
        res %= p;
    }
    return res;
}