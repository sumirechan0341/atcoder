use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        mut n: i32,
        k: i32
    };
    for i in 1.. {
        n /= k;
        if n == 0 {
            println!("{}", i);
            return;
        }
    }
}