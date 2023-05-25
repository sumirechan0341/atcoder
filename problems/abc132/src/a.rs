use proconio::{input, marker::Chars};
use std::collections::HashMap;
pub fn main() {
    input! {
        s: Chars
    };
    let mut dict: HashMap<char, i32> = HashMap::new();
    for c in s {
      dict.insert(c, dict.get(&c).unwrap_or(&0) + 1);
    }
    println!("{}", if dict.len() == 2 && dict.values().all(|&x| x == 2) { "Yes" } else { "No" });
}