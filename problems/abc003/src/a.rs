use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        n: f32
    };
    println!("{}", (n + 1_f32) / 2_f32 * 10000_f32);
}