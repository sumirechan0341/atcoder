use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
        c: String
    };
    if c == "Red".to_string() {
        println!("{}", g.min(b));
    }
    if c == "Blue".to_string() {
        println!("{}", g.min(r));
    }
    if c == "Green".to_string() {
        println!("{}", r.min(b));
    }
}
