use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: Chars
    };
    println!("{}", if n.contains(&'7') { "Yes" } else { "No" });
}