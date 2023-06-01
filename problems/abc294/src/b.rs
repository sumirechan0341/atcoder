use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u8; w]; h]
    };
    let offset: u8 = 64;
    println!("{}", a.iter().map(|aw| aw.iter().map(|c| if c == &0 { '.' } else { (offset + c) as char }).collect::<String>()).collect::<Vec<_>>().join("\n"));
}