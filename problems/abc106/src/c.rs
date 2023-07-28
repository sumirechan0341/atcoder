use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars,
        k: usize
    };
    let mut index = 0;
    if let Some(i) = s.iter().position(|&c| c != '1') {
        index = i
    }
    if k > index {
        println!("{}", s[index]);
    } else {
        println!("{}", "1");
    }
}