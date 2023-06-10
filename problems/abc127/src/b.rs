use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        r: i64,
        d: i64,
        mut x: i64
    };
    for _i in 1..11 {
        x = r * x - d;
        println!("{}", x);
    }
}