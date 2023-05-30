use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        c: char
    };
    println!("{}", if c == 'a' || c == 'i' || c == 'u' || c == 'e' || c == 'o' { "vowel" } else { "consonant" });
}