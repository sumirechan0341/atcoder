use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32
    };
    println!("{}", (n-2)*180);
}