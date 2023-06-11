use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        mut ln: [i32; n]
    };
    ln.sort();
    println!("{}", if ln[n-1] < ln[0..n-1].iter().sum::<i32>() { "Yes" } else { "No" });
}
