use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut s: Chars,
        mut t: Chars
    };
    s.sort();
    t.sort();
    t.reverse();
    println!("{}", if s.into_iter().collect::<String>() < t.into_iter().collect::<String>() { "Yes" } else { "No" } );
}