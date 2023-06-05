use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        mut a: u64,
        b: u64,
        k: u64
    };
    let mut count = 0;
    while a < b {
        a = a*k;
        count += 1;
    }
    println!("{}", count);
}