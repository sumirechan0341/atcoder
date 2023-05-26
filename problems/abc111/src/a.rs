use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    println!("{}", s.iter().map(|&c| match c { '1' => '9', '9' => '1', x => x }).collect::<String>());
}