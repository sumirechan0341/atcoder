use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n]
    };
    for i in 0..n - 1 {
        println!("{}", an[i] * an[i + 1]);
    }
}
