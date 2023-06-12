use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        d: u32,
        n: i32
    };
    println!("{}", 100_i32.pow(d) * n + if n == 100 { 100_i32.pow(d) } else { 0 });
}