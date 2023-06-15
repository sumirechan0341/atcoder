use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        x: i32
    };
    for i in 1.. {
        if i*i*i*i == x {
            println!("{}", i);
            return;
        }
    }
}