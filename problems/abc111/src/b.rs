use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: i32
    };
    println!("{}", ((n + 110) / 111) * 111);
}