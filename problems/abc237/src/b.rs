use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        h: usize,
        w: usize,
        ahw: [[i32; w]; h]
    };
    let mut t = vec![vec![0; h]; w];
    for i in 0..h {
        for j in 0..w {
            t[j][i] = ahw[i][j];
        }
    }
    for i in 0..w {
        println!("{}", t[i].iter().map(|x| x.to_string()).collect::<VS>().join(" "));
    }
}