use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
        w: usize,
        mut shw: [Chars; h]
    };
    for i in 0..h {
        for j in 0..w-1 {
            if shw[i][j] == 'T' && shw[i][j+1] == 'T' {
                shw[i][j] = 'P';
                shw[i][j+1] = 'C';
            }
        }
    }
    for i in 0..h {
        println!("{}", shw[i].iter().collect::<String>());
    }
}