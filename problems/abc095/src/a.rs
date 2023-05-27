use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        x: Chars
    }
    println!("{}", 700 + 100 * x.iter().filter(|&c| c == &'o').collect::<Vec<_>>().len());
}