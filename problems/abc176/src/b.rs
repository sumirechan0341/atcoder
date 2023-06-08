use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: Chars
    };
    let mut d = 0;
    for c in n {
        d += c.to_digit(10).unwrap();
    }
    println!("{}", if d % 9 == 0 { "Yes" } else { "No" });
}