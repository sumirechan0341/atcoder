use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    for i in 0..s.len() {
        if i%2 == 1 && s[i] != '0' {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}