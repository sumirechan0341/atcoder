use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        lrn: [(i32, i32); n]
    };
    println!("{}", lrn.into_iter().map(|(l, r)| r - l + 1).sum::<i32>());
}