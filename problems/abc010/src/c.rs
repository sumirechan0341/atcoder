use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        txy1: (f32, f32),
        txy2: (f32, f32),
        t: f32,
        v: f32,
        n: usize,
        xyn: [(f32, f32); n]
    };
    for &(x, y) in &xyn {
        if norm2(txy1, (x, y)) + norm2(txy2, (x, y)) <= t*v {
            println!("{}", "YES");
            return;
        }
    }
    println!("{}", "NO");
}
fn norm2(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    return ((p1.0-p2.0).powf(2.0) + (p1.1-p2.1).powf(2.0)).sqrt();
}