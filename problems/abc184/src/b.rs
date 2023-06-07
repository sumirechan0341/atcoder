use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        mut x: i32,
        s: Chars
    };
    for i in 0..n {
        x += if s[i] == 'o' { 1 } else { -1 };
        x = x.max(0);
    }
    println!("{}", x);
}