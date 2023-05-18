use proconio::input;

pub fn main() {
    input! {
        x: i32,
        y: i32
    };
    println!("{}", if x >= y { 0 } else { ((y - x) as f32 / 10.0).ceil() as i32 });
}