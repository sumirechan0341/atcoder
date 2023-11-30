use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n],
        q: usize,
        queryq: [(usize, usize, usize); q]
    };
    let mut count = vec![vec![]; 200001];
    for i in 0..n {
        count[an[i]].push(i + 1);
    }
    for (l, r, x) in queryq {
        println!(
            "{}",
            count[x].partition_point(|&y| y <= r) - count[x].partition_point(|&y| y < l)
        );
    }
}
