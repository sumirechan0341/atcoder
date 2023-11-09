use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        b: i64
    };
    for i in 1i64..=16 {
        if i.pow(i as u32) == b {
            println!("{}", i);
            return
        }
    }
    println!("{}", -1);
}