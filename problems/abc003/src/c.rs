use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        mut rn: [i32; n]
    };
    rn.sort();
    println!("{}", rn[n-k..n].iter().fold(0.0, |acc, x| (acc + *x as f64)/2.0));
}