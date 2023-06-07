use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64
    };
    
    println!("{}", if c*d-b > 0 { (a + c*d-b - 1) / (c*d-b) } else { -1 });
}