use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        q: i32
    };
    println!("{}", if q == 1 { "ABC" } else { "chokudai" });
}