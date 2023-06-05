use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: i128
    };
    println!("{}", if n > 0 { n / 10 } else { (n - 9) / 10 });
}