use std::f64::consts::PI;

use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        a: f64,
        b: f64,
        d: f64
    };
    let rad = d / 180.0 * PI;
    println!("{} {}", a * rad.cos() - b * rad.sin(), a * rad.sin() + b *rad.cos());
}