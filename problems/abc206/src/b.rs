use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32
    };
    for i in 1.. {
        if i * (i+1) / 2 >= n {
            println!("{}", i);
            return;
        }
    }
}