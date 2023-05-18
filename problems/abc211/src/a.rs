use proconio::input;

pub fn main() {
    input !{
        a: f32,
        b: f32
    };
    println!("{}", (a - b) / 3.0_f32 + b);
}