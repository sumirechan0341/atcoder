use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        mut xm: [i64; m]
    };
    xm.sort();
    let mut diff = vec![0; m-1];
    for i in 0..m-1 {
        diff[i] = xm[i+1] - xm[i];
    }
    diff.sort();
    diff.reverse();
    if n >= m {
        println!("{}", 0);
        return;
    }
    println!("{}", diff[(n-1)..].iter().sum::<i64>());
}