use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        a: char,
        b: char
    };
    let l = a.max(b);
    let s = a.min(b);
    for _ in 0..l.to_digit(10).unwrap() {
        print!("{}", s);
    }
    println!("{}", "");
}