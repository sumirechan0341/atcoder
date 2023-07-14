use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64
    };
    let rad_short = (h * 30.0 + 0.5 * m) / 180.0 * std::f64::consts::PI;
    let rad_long = 6.0 * m / 180.0 * std::f64::consts::PI;
    let p1 = (b * rad_short.cos(), b * rad_short.sin());
    let p2 = (a * rad_long.cos(), a * rad_long.sin());
    println!("{}", ((p1.0-p2.0).powf(2.0)+(p1.1-p2.1).powf(2.0)).sqrt());
}