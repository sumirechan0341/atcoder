use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: u128,
        b: u128,
        c: u128,
        d: u128,
        e: u128,
        f: u128
    };
    let abc = vec![a, b, c].iter().fold(1, |acc, x| (acc * (x % 998244353)) % 998244353);
    let def = vec![d, e, f].iter().fold(1, |acc, x| (acc * (x % 998244353)) % 998244353);
    println!("{}", (abc-def+998244353) % 998244353);
}