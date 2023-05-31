use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: Chars,
        b: Chars
    };
    println!("{}", if a.len() > b.len() { a.iter().collect::<String>() } else { b.iter().collect::<String>() });
}