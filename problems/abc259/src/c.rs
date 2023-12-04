use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut s: Chars,
        t: Chars
    };
    if s.len() > t.len() {
        println!("{}", "No");
        return;
    }
    for i in 0..t.len() {
        if i == s.len() {
            if s[i - 1] == t[i] && s[i - 2] == t[i] {
                s.insert(i, t[i]);
            } else {
                println!("{}", "No");
                return;
            }
        }
        if s[i] != t[i] {
            if i < 2 {
                println!("{}", "No");
                return;
            }
            if s[i - 1] == t[i] && s[i - 2] == t[i] {
                s.insert(i, t[i]);
            } else {
                println!("{}", "No");
                return;
            }
        }
    }
    println!("{}", "Yes");
}
