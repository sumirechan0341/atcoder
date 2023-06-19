use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: Chars,
        t: Chars
    };
    for i in 0..s.len() {
        if s[i] != t[i] {
            println!("{}", i+1);
            return;
        }
    }
    println!("{}", t.len());
}