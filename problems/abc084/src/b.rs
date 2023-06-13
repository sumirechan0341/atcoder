use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: usize,
        b: usize,
        mut s: Chars
    };
    if s[a] != '-' {
        println!("{}", "No");
        return;
    }
    s.remove(a);
    println!("{}", if s.into_iter().all(|c| c.is_digit(10)) { "Yes" } else { "No" });
}