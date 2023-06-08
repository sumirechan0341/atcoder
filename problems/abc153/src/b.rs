use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        h: i32,
        n: usize,
        an: [i32; n]
    };
    println!("{}", if h <= an.iter().sum::<i32>() { "Yes" } else { "No" } );
}