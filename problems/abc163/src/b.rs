use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: i32,
        m: usize,
        am: [i32; m]
    };
    println!("{}", (n-am.iter().sum::<i32>()).max(-1));
}