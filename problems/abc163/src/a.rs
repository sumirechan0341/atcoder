use std::f32::consts::PI;
use proconio::input;

pub fn main() {
    input! {
        r: f32
    };
    println!("{}", 2.0 * PI * r);
}