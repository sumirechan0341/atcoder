use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        s: Chars
    };
    let mut count = 0;
    for i in 0..n-2 {
        if s[i..i+3].into_iter().collect::<String>() == "ABC" {
            count += 1;
        }
    }
    println!("{}", count);
}