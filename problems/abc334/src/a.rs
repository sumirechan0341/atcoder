use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        b: usize,
        g: usize
    };
    println!("{}", if b > g { "Bat" } else { "Glove" });
}
