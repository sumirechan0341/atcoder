use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n]
    };
    let mut count: u64 = 0;
    let mut combn: u64 = 0;
    for i in 0..n {
        if i+1 == an[i] {
            combn += 1;
            continue;
        }
        if i+1 == an[an[i]-1] {
            count += 1;
        }
    }
    count /= 2;
    count += (combn * (combn-1)) / 2;
    println!("{}", count);
}