use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    for i in 0..s.len() {
        if (i % 2 == 0 && s[i] == 'L') || (i % 2 == 1 && s[i] == 'R') {
            println!("{}", "No");
            return;
        } 
    }
    println!("{}", "Yes");
}