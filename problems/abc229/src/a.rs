use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s1: Chars,
        s2: Chars,
    };
    if s1.iter().filter(|&c| c == &'#').count() + s2.iter().filter(|&c| c == &'#').count() > 2 {
      println!("{}", "Yes");
      return
    }
    println!("{}", if s1[0] == '#' && s2[1] == '#' || s1[1] == '#' && s2[0] == '#' { "No" } else { "Yes" });
}