use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i64
    };
    let mut p = n;
    let mut q = 1;
    for i in 2.. {
        if i*i > n {
            break;
        }
        if n % i == 0 {
            p = n / i;
            q = i;
        }
    }
    println!("{}", p + q - 2);
}