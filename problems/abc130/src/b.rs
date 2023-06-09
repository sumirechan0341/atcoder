use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        x: i32,
        ln: [i32; n]
    };
    let mut d = 0;
    for i in 1.. {
        if i == n + 1 {
            println!("{}", i);
            return;
        }
        d += ln[i-1];
        if d > x {
            println!("{}", i);
            return;
        }
    }
}