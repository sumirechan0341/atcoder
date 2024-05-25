use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut lrn: [(usize, usize); n]
    };
    lrn.sort();
    let mut ans = 0;
    for i in 0..n {
        let right = lrn[i].1;
        ans += lrn[i + 1..].partition_point(|x| x.0 <= right);
    }
    println!("{}", ans);
}
