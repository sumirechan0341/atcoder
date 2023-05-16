use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
pub fn main() {
    input !{
        s: Chars
    };
    let mut bucket = HashMap::new();
    for c in &s {
        bucket.insert(c, 0);
    }
    for c in &s {
        bucket.insert(c, bucket.get(c).unwrap() + 1);
    }
    for item in bucket.iter() {
        if *item.1 == 1 {
            println!("{}", item.0);
            return
        }
    }
    println!("{}", -1);
}