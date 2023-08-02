use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i64,
        m: i64
    };
    if n > 2 && m > 2 {
        println!("{}", (n-2)*(m-2));
        return;
    }
    if n == 2 || m == 2 {
        println!("{}", 0);
        return;
    }
    println!("{}", (n*m-2).max(1));
}