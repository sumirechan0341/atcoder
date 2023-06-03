use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut n: i128
    };
    while n < 0 {
        n += 998244353;
    }
    println!("{}", n % 998244353);
}