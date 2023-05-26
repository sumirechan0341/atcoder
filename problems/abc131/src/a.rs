use proconio::input;
use proconio::marker::Chars;
pub fn main() {
    input! {
        s: Chars
    };
    let mut prev = s[0];
    for i in 0..3 {
        if prev == s[i + 1] {
          println!("{}", "Bad");
          return;
        }
        prev = s[i + 1];
    }
    println!("{}", "Good");
}