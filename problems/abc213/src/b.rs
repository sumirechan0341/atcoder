use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n]
    };
    let mut sorted = an.iter().enumerate().collect::<Vec<_>>();
    sorted.sort_by_key(|a| a.1);
    println!("{}", sorted[n-2].0 + 1);
}