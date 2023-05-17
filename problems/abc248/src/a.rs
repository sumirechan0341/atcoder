use proconio::input;
use proconio::marker::Chars;
pub fn main() {
    input! {
        s: Chars
    };
    for i in 0..10 {
        if !s.contains(&i.to_string().chars().nth(0).unwrap()) {
          println!("{}", i);
          return
        }
    }
}