use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: i64
    };
    let mut dict = vec![];
    for i in 1..10_i64.pow(5) {
        dict.push(i*i);
    }
    println!("{}", dict.into_iter().filter(|x| x <= &n).rev().collect::<Vec<_>>()[0]);
}