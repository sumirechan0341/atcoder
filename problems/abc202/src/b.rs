use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    println!("{}", s.into_iter().map(kaiten).rev().collect::<String>());
}
fn kaiten(c: char) -> char {
    match c {
        '0' | '1' | '8' => c,
        '6' => '9',
        '9' => '6',
        _ => c
    }
}