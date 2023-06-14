use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        h: usize,
        w: usize,
        chw: [Chars; h]
    };
    for i in 0..h {
        for k in 0..2 {
            for j in 0..w {
                print!("{}", chw[i][j]);
            }
            println!("{}", "");
        }   
    }
}