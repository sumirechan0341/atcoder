use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [i64; n]
    };
    let mut aan = an.clone();
    aan.sort();
    let mut sn = vec![0; n + 1];
    for i in 0..n {
        sn[i + 1] = sn[i] + aan[i];
    }
    for i in 0..n {
        println!("{}", sn[n] - sn[aan.partition_point(|x| x < &(an[i] + 1))]);
    }
}
