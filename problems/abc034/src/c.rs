use std::mem::swap;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        w: usize,
        h: usize
    };
    println!("{}", mod_comb((w+h-2) as i64, (w-1) as i64, 1000000007_i64));
}

fn mod_comb(n: i64, r: i64, p: i64) -> i64 {
    let mut result = 1;
    for i in 0..r {
        result *= (n-i)*modinv(i+1, p) % p;
        result %= p;
    }
    return result;
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