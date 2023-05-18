use proconio::input;
use proconio::marker::Chars;
pub fn main() {
    input! {
        mut s: Chars,
        a: usize,
        b: usize
    };
    let temp = s[a-1];
    s[a-1] = s[b-1];
    s[b-1] = temp;
    println!("{}", s.into_iter().collect::<String>());
}