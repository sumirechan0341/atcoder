use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        mut an: [i32; n]
    };
    an.sort();
    println!("{}", an[n-1] - an[0]);
}