use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        sn: [Chars; n]
    };
    for i in 0..n {
        for j in 0..n {
            let mut row = 0;
            let mut col= 0;
            let mut diag1 = 0;
            let mut diag2 = 0;
            for k in 0..6 {
                if j+k < n {
                    if sn[i][j+k] == '#' {
                        row += 1;
                    }
                } else {
                    // invalid
                    row = 0;
                }
                if i+k < n {
                    if sn[i+k][j] == '#' {
                        col += 1;
                    }
                }
                else {
                    // invalid
                    col = 0;
                }
                if j+k < n && i+k < n {
                    if sn[i+k][j+k] == '#' {
                        diag1 += 1;
                    }
                } else {
                    // invalid
                    diag1 = 0;
                }
                if i+k < n && k <= j {
                    if sn[i+k][j-k] == '#' {
                        diag2 += 1;
                    }
                } else {
                    // invalid
                    diag2 = 0;
                }
            }
            if row >= 4 || col >= 4 || diag1 >= 4 || diag2 >= 4 {
                println!("{}", "Yes");
                return;
            }
        }
    }
    println!("{}", "No");
}