use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i32,
        b: i32
    };
    println!("{}", if a % 3 == 0 || b % 3 == 0 || (a + b) % 3 == 0 { "Possible" } else { "Impossible" });
}