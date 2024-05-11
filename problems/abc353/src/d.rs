use std::collections::HashMap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n]
    };
    let m = 998244353;
    let mut invs = vec![0; 2000000];
    invs[0] = 1;
    for i in 0..1999999 {
        invs[i + 1] = invs[i] * 10 % m;
    }
    let digit = an
        .iter()
        .map(|x| x.to_string().len())
        .collect::<Vec<usize>>();
    let mut e = 0;
    for i in 1..n {
        e += 10_usize.pow(digit[i] as u32);
        e %= m;
    }
    let mut ans = 0;
    for i in 0..n - 1 {
        ans += e * an[i] % m;
        ans %= m;
        e = (e + m - (10_usize.pow(digit[i + 1] as u32) % m)) % m;
        ans += an[i] * i;
        ans %= m;
    }
    ans += an[n - 1] * (n - 1);
    ans %= m;
    println!("{}", ans);
}
