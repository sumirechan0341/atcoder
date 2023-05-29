use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        n: Chars
    };
    println!("{}", if n.contains(&'9') { "Yes" } else { "No" });
}