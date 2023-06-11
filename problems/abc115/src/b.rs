use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        mut pn: [i32; n]
    };
    pn.sort();
    println!("{}", pn[n-1] / 2 + pn[0..n-1].iter().sum::<i32>());
}