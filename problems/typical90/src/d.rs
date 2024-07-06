use itertools::Itertools;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [[usize; w]; h]
    };
    let mut sh = vec![0; h];
    for i in 0..h {
        for j in 0..w {
            sh[i] += ahw[i][j];
        }
    }
    let mut sw = vec![0; w];
    for j in 0..w {
        for i in 0..h {
            sw[j] += ahw[i][j];
        }
    }
    for i in 0..h {
        println!("{}", (0..w).map(|j| sh[i] + sw[j] - ahw[i][j]).join(" "));
    }
}
