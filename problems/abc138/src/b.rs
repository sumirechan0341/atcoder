use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [f64; n]
    };
    println!("{}", 1.0 / an.iter().map(|x| 1.0 / x).sum::<f64>());
}