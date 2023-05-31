use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        w: i32,
        h: i32
    };
    println!("{}", if w * 3 == h * 4 { "4:3" } else { "16:9" });
}