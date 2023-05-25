use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars,
        t: Chars
    };
    let mut count = 0;
    for i in 0..3 {
        if s[i] == t[i] {
          count += 1;
        }
    }
    println!("{}", count);
}