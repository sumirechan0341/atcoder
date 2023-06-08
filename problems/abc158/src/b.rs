use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: u64,
        a: u64,
        b: u64
    };
    println!("{}", n / (a + b) * a + (n % (a + b)).min(a)); 
}