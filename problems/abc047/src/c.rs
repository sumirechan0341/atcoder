use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: Chars
    };
    let mut total = 0;
    let mut prev = s[0];
    for i in 1..s.len() {
        if prev != s[i] {
            total += 1;
            prev = s[i];
        }
    }
    println!("{}", total);
}