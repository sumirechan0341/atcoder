use proconio::input;

pub fn main() {
    input! {
        h: f64
    };
    println!("{}", ((12800000_f64 + h) * h).sqrt());
}