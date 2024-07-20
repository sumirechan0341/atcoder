use std::collections::BinaryHeap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut lrn: [(i64, i64); n]
    };
    let mut lsum = 0;
    let mut rsum = 0;

    for i in 0..n {
        lsum += lrn[i].0;
        rsum += lrn[i].1;
    }
    if lsum > 0 || rsum < 0 {
        println!("{}", "No");
        return;
    }
    let mut result = lrn.iter().map(|x| x.0).collect::<Vec<_>>();
    let mut adj = -lsum;
    for i in 0..n {
        let max_inc = lrn[i].1 - lrn[i].0;
        let inc = adj.min(max_inc);
        result[i] += inc;
        adj -= inc;
        if adj == 0 {
            break;
        }
    }
    println!("{}", "Yes");
    println!(
        "{}",
        result
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
