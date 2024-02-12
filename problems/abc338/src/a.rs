use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars,
    };
    for i in 0..s.len() {
        if i == 0 && !s[i].is_uppercase() {
            println!("{}", "No");
            return;
        }
        if i != 0 && !s[i].is_lowercase() {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}
