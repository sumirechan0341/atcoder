use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        mut an: [i32; n]
    };
    if n == 1 {
        println!("{}", 0);
        return;
    }
    println!("{}", (an[1..].iter().max().unwrap()-an[0]+1).max(0));
}