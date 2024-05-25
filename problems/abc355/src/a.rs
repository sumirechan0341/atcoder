use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i32,
        b: i32
    };
    if a != b {
        println!("{}", 6 - (a + b));
    } else {
        println!("{}", -1);
    }
}
