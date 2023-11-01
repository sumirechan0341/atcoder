use proconio::{input, marker::Chars};
use std::mem::swap;
pub fn main() {
    input! {
        n: usize,
        an: [i64; n]
    };
    let p = 998244353;
    let n_inv = modinv(n as i64, p);
    let mut total = n_inv;
    let mut ans = 0;
    for i in 0..n {
        ans += (total*an[i])%p;
        ans %= p;
        total += total*n_inv;
        total %= p;
    }
    println!("{}", ans);
}

fn modinv(i: i64, m: i64) -> i64 {
    let a = &mut i.clone();
    let b = &mut m.clone();
    let u = &mut 1;
    let v = &mut 0;
    while *b > 0 {
        let t = *a / *b;
        *a -= t * (*b);
        swap(a, b);
        *u -= t * (*v); 
        swap(u, v);
    }
    *u %= m; 
    if *u < 0 {
        *u += m;
    }
    return *u;
}