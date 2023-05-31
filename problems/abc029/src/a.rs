use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        w: String
    };
    println!("{}", w + &"s");
}