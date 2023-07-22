use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        w: i64,
        h: i64,
        x: i64,
        y: i64
    };
    
    println!("{} {}", (w*h) as f64 / 2.0, if 2*x == w && 2*y == h { 1 } else { 0 });
}