use proconio::input;

pub fn main() {
    input! {
        x: f32
    };
    println!("{}", (x / 100.0_f32).ceil());
}