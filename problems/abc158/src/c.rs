use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        a: i32,
        b: i32,
    };
    for i in 13..1010 {
        if a == (i*8) / 100 && b == (i*10) / 100 {
            println!("{}", i);
            return;
        }
    }
    println!("{}", -1);
}