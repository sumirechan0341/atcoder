use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: char,
        t: char
    };
    println!("{}", if s == 'Y' { t.to_ascii_uppercase() } else { t });
}