use proconio::input;

pub fn main() {
    input! {
        a: f32,
        b: f32
    };
    println!("{}", (a - b) / a * 100.0);
}