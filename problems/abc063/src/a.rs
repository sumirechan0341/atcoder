use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i32,
        b: i32
    };
    println!("{}", if a + b < 10 { (a + b).to_string() } else { "error".to_string() });
}