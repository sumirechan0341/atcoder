use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        a: f32,
        b: f32
    };
    let d = (a.powf(2.0) + b.powf(2.0)).sqrt();
    println!("{} {}", a / d, b / d);
}