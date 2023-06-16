use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        w: Chars
    };
    println!("{}", w.iter().filter(|&x| x != &'a' && x != &'i' && x != &'u' && x != &'e' && x != &'o').collect::<String>());
}