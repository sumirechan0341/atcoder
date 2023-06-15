use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32,
        m: f32
    };
    println!("{}", (30.0*(n % 12) as f32 + 0.5*m - 6.0*m).abs().min(360.0 - (30.0*(n % 12) as f32 + 0.5*m - 6.0*m).abs()));
}