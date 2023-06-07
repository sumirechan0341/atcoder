use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        s: i32,
        d: i32,
        xyn: [(i32, i32); n]
    };
    println!("{}", if xyn.iter().filter(|x| x.0 < s && x.1 > d).collect::<Vec<_>>().len() > 0 { "Yes" } else { "No" });
}