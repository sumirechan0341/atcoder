use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        k: usize,
        s: Chars
    };
    println!("{}", if s.len() > k { s[..k].iter().collect::<String>() + "..." } else { s.iter().collect::<String>() });
}