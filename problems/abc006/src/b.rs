use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32
    };
    if n < 3 {
        println!("{}", 0);
        return;
    }
    let mut a0 = 0;
    let mut a1 = 0;
    let mut a2 = 1;
    for i in 3..n {
        let a3 = (a0 + a1 + a2) % 10007;
        a0 = a1;
        a1 = a2;
        a2 = a3;
    }
    println!("{}", a2);
}