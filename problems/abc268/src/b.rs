use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars,
        t: Chars
    };
    if s.len() > t.len() {
        println!("{}", "No");
        return
    }
    for i in 0..s.len() {
        if s[i] != t[i] {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}