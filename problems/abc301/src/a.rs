use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        s: Chars
    };
    let mut t = 0;
    let mut a = 0;
    for c in s {
      if c == 'T' {
        t += 1;
      } else {
        a += 1;
      }
      if t == (n + 1) / 2 {
        println!("{}", "T");
        return;
      } 
      if a == (n + 1) / 2 {
        println!("{}", "A");
        return;
      }
    }
}