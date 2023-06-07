use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        h: usize,
        w: usize,
        ahw: [[i32; w]; h]
    };
    let mut min = 100;
    for i in 0..h {
        for j in 0..w {
            if min > ahw[i][j] {
                min = ahw[i][j];
            }
        }
    }
    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            count += ahw[i][j] - min;
        }
    }
    println!("{}", count);
}