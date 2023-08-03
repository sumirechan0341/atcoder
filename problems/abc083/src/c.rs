use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        mut x: i64,
        y: i64
    };
    for i in 1.. {
        x *= 2;
        if x > y {
            println!("{}", i);
            return;
        }
    }
}