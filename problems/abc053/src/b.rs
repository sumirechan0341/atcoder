use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    let mut start = 0;
    let mut end = 0;
    for i in 0..s.len() {
        if start == 0 && s[i] == 'A' {
            start = i+1;
        }
        if s[i] == 'Z' {
            end = i+1;
        }
    }
    println!("{}", end-start+1);
}