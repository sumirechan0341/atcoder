use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        h: usize,
        w: usize,
        ahw: [[i32; w]; h]
    };
    for i in 0..h-1 {
        for j in 0..w-1 {
            for xstride in 1..h-i {
                for ystride in 1..w-j {
                    if ahw[i][j] + ahw[i+xstride][j+ystride] > ahw[i+xstride][j] + ahw[i][j+ystride] {
                        println!("{}", "No");
                        return;
                    }
                }
            }
        }
    }
    println!("{}", "Yes");
}