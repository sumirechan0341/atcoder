use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars
    };
    let mut pre = true;
    let mut post = true;
    for i in 0..n {
        if s[i] != t[i] {
            pre = false;
        }
    }
    for i in 0..n {
        if s[s.len()-1-i] != t[t.len()-1-i] {
            post = false;
        }
    }
    if pre && post {
        println!("{}", 0);
    } else if pre {
        println!("{}", 1);
    } else if post {
        println!("{}", 2);
    } else {
        println!("{}", 3);
    }
}