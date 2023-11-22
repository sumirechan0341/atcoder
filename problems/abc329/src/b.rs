use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [i32; n]
    };
    let max = an.iter().max().unwrap();
    let mut next = 0;
    for i in 0..n {
        if an[i] == *max {
            continue;
        }
        if an[i] > next {
            next = an[i];
        }
    }
    println!("{}", next);
}