use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut x: f64,
        k: f64
    };
    for i in 0..(k as u32) {
        let r = 10_i64.pow(i + 1) as f64;
        x = (x / r).round() * r;
    }
    println!("{}", x);
}