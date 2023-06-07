use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        p26: [u8; 26]
    };

    for p in p26.into_iter().map(|c| (c + 96) as char) {
        print!("{}", p);
    }
    println!("{}", "");
}