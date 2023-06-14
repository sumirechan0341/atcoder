use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [Chars; h]
    };
    let mut ans = vec![vec!['#'; w+2]; h+2];
    for i in 0..h {
        for j in 0..w {
            ans[i+1][j+1] = ahw[i][j];
        }
    }
    for i in 0..h+2 {
        println!("{}", ans[i].iter().collect::<String>());
    }
}