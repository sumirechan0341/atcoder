use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        r: usize
    };
    println!("{}", 100 - r % 100);
}
