use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        an: [i32; 4*n-1]
    };
    println!("{}", an[1..].iter().fold(an[0], |acc, x| acc ^ x));
}