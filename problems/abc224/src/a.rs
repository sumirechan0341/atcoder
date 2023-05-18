use proconio::{input};
pub fn main() {
    input! {
        s: String
    };
    println!("{}", if &s[s.len()-2..s.len()] == "er" { "er" } else { "ist" });
}