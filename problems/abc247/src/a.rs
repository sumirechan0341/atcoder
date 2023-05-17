use proconio::input;
use proconio::marker::Chars;
pub fn main() {
    input! {
        mut s: Chars
    };
    for i in 0..3 {
      s[3-i] = s[2-i];
    }
    s[0] = '0';
    println!("{}", s.iter().collect::<String>());
}