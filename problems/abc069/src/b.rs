use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    println!("{}", s[0].to_string() + &(s.len()-2).to_string() + &s[s.len()-1].to_string());
}