use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: Chars,
        b: Chars,
        c: Chars
    };
    println!("{}", if a[a.len() - 1] == b[0] && b[b.len() - 1] == c[0] { "YES" } else { "NO" });
}