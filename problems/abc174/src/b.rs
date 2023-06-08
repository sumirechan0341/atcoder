use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        d: i64,
        xyn: [(i64, i64); n]
    };
    println!("{}", xyn.into_iter().filter(|&xy| norm2(xy) <= d.pow(2)).collect::<Vec<_>>().len());
}

fn norm2(p: (i64, i64)) -> i64 {
    return p.0.pow(2) + p.1.pow(2);
}