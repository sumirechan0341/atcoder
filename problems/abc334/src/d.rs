use itertools::Itertools;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        q: usize,
        mut rn: [usize; n],
    };
    rn.sort();
    let mut sn = vec![0; n + 1];
    for i in 0..n {
        sn[i + 1] = sn[i] + rn[i];
    }
    let mut ans = vec![];
    for _ in 0..q {
        input! {
            query: usize
        }
        ans.push(sn.partition_point(|&x| x <= query) - 1);
    }
    println!("{}", ans.iter().join("\n"));
}
