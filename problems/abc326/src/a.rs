use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        x: i32,
        y: i32
    };
    if x >= y {
        println!("{}", if x-y <= 3 { "Yes" } else { "No" });
    } else {
        println!("{}", if y-x <= 2 { "Yes" } else { "No" });
    }
}