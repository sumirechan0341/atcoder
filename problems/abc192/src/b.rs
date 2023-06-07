use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    for i in 0..s.len() {
        if i % 2 == 1 {
            if !s[i].is_uppercase() {
                println!("{}", "No");
                return
            }
        } else {
            if !s[i].is_lowercase() {
                println!("{}", "No");
                return
            }
        }
    }
    println!("{}", "Yes");
}