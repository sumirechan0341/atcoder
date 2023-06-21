use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: i64
    };
    let mut total = 1;
    let mut prev = 1;
    for i in 1..=n {
        prev = total - prev;
        total += prev;
    }
    println!("{}", total);
}