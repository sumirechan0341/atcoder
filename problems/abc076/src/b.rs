use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: i32,
        k: i32
    };
    let mut d = 1;
    for i in 0..n {
        if d < k {
            d *= 2;
        } else {
            d += k;
        }
    }
    println!("{}", d);
}