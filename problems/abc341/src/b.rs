use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut an: [i64; n],
        st1: [(i64, i64); n-1]
    };
    for i in 0..n - 1 {
        an[i + 1] += (an[i] / st1[i].0) * st1[i].1;
    }
    println!("{}", an[n - 1]);
}
