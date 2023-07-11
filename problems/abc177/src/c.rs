use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i64; n]
    };
    let p = 1000000007;
    let s1 = an.iter().sum::<i64>() % p;
    let s2 = an.iter().map(|x| (x*x) % p).sum::<i64>() % p;
    println!("{}", (((s1*s1) % p + p - s2) * 500000004) % p);
}