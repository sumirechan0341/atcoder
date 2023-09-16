use proconio::{input, marker::Chars, input_interactive};

pub fn main() {
    input! {
        a: u32,
        b: u32
    };
    println!("{}", a.pow(b)+b.pow(a));
}