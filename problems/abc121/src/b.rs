use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        m: usize,
        c: i32,
        bm: [i32; m],
        anm: [[i32; m]; n]
    };
    let mut count = 0;
    for i in 0..n {
        let mut left_val= c;
        for j in 0..m {
            left_val += anm[i][j] * bm[j];
        }
        if left_val > 0 {
            count += 1;
        }
    }
    println!("{}", count);
}