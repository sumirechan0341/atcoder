use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s1: char,
        s2: char,
        s3: char,
        t1: char,
        t2: char,
        t3: char
    };
    let mut count = 0;
    if s1 != t1 {
        count += 1;
    }
    if s2 != t2 {
        count += 1;
    }
    if s3 != t3 {
        count += 1;
    }
    println!("{}", if count == 0 || count == 3 { "Yes" } else { "No" });
}