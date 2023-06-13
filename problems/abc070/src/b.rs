use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        a: i32,
        b: i32,
        c: i32,
        d: i32
    };
    let mut count = 0;
    for i in 0..101 {
        if a <= i && i < b && c <= i && i < d {
            count += 1;
        }
    }
    println!("{}", count);
}