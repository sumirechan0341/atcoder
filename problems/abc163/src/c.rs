use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n-1]
    };
    let mut member = vec![0; n];
    for i in 0..n-1 {
        member[an[i]-1] += 1;
    }
    for i in 0..n {
        println!("{}", member[i]);
    }
}