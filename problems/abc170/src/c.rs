use proconio::{input, marker::Chars};
use std::io::stdin;
type VS = Vec<String>;

pub fn main() {
    input!{
        x: i32,
        n: usize,
        pn: [i32; n]
    };
    let mut target = vec![];
    for i in 0..=101 {
        if !pn.contains(&i) {
            target.push(i);
        }
    }
    target.sort_by(|a, b| (a-x).abs().cmp(&(b-x).abs()).then(a.cmp(b)));
    println!("{}", target[0]);
}