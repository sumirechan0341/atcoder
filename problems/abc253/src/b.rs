use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        h: usize,
        w: usize,
        sh: [Chars; h]
    };
    let mut p1 = (-1, -1);
    let mut p2 = (-1, -1);
    for i in 0..h {
        for j in 0..w {
            if sh[i][j] == 'o' {
                if p1.0 == -1 {
                    p1 = (i as i32, j as i32);
                } else {
                    p2 = (i as i32, j as i32);
                }
            }
        }
    }
    println!("{}", (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs());
}