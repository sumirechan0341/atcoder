use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        s: Chars
    };
    println!("{}", s.iter().filter(|&c| c == &'1').collect::<Vec<_>>().len());
}