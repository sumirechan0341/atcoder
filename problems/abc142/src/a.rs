use proconio::input;

pub fn main() {
    input! {
        n: f32
    };
    println!("{}", (n / 2.0_f32).ceil() / n);
}