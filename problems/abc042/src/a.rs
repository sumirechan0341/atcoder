use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        a: i32,
        b: i32,
        c: i32
    };
    match (a, b, c) {
        (5, 5, 7) | (5, 7, 5) | (7, 5, 5) => println!("{}", "YES"),
        _ => println!("{}", "NO")
    }
}