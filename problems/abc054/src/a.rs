use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i32,
        b: i32
    };
    println!("{}", if a == b { "Draw" } else if a == 1 { "Alice" } else if b == 1 { "Bob" } else if a > b { "Alice" } else { "Bob" });
}