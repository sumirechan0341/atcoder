use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        a: u64,
        b: u64,
        c: u64,
    };
    let p = 998244353;
    let suma = (a*(a+1)/2)%p;
    let sumb = (b*(b+1)/2)%p;
    let sumc = (c*(c+1)/2)%p;
    println!("{}", (((suma * sumb) % p) * sumc) % p);
}