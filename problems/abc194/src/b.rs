use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut abn: [(i32, i32); n]
    };
    let mut time = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            time[i][j] = if i == j { abn[i].0 + abn[i].1 } else { abn[i].0.max(abn[j].1) }
        }
    }
    let mut min = 10_i32.pow(5);
    for i in 0..n {
        for j in 0..n {
            if min > time[i][j] {
                min = time[i][j];
            }
        }
    }
    println!("{}", min);
}