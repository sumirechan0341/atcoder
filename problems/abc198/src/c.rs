use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        r: i64,
        x: i64,
        y: i64
    };
    for i in 1..=10_i64.pow(7) {
        if x*x+y*y <= i*i*r*r {
            if i == 1 && x*x+y*y != i*i*r*r {
                println!("{}", 2);
                return;
            } else {
                println!("{}", i);
                return;
            }
        }
    }
}