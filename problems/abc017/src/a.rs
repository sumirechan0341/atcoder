use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        se: [[i32; 2]; 3],
    };
    println!("{}", se.iter().map(|z| z[0] * z[1] / 10).sum::<i32>());
}