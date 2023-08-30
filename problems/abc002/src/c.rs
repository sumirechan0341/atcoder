use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    };
    println!("{}", ((x2-x1)*(y3-y1)-(y2-y1)*(x3-x1)).abs()/2.0);
}