use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        x: i32,
        y: i32
    };
    println!("{}", if y > x { "Better" } else { "Worse" });
}