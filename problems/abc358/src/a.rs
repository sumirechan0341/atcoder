use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: String,
        t: String
    };
    if s == "AtCoder".to_string() && t == "Land".to_string() {
        println!("Yes");
    } else {
        println!("No");
    }
}
