use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i64
    };
    let p = 1000000007;
    let mut pow = 1;
    for i in 1..=n {
        pow *= i;
        pow %= p;
    }
    println!("{}", pow);
}