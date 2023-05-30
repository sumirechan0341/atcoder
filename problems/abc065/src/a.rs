use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        x: i32,
        a: i32,
        b: i32
    };
    let diff = a - b;
    println!("{}", if diff >= 0 { "delicious" } else if -diff <= x  { "safe" } else { "dangerous" });
}