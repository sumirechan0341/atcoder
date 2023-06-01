use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut x: [i32; 5 * n]
    };
    x.sort();
    println!("{}", (x[n..4*n].iter().sum::<i32>()) as f32 / (3.0 * n as f32));
}