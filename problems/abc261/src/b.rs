use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        n: usize,
        ann: [Chars; n]
    };
    for i in 0..n {
        for j in i..n {
            if i == j {
                if ann[i][j] != '-' {
                    println!("{}", "incorrect");
                    return;
                }
                continue;
            }
            // i != j
            if ann[i][j] == '-' || ann[j][i] == '-' {
                println!("{}", "incorrect");
                return;
            }
            if ann[i][j] == 'D' && ann[j][i] != 'D' {
                println!("{}", "incorrect");
                return;
            }
            if ann[i][j] == 'W' && ann[j][i] != 'L' {
                println!("{}", "incorrect");
                return;
            }
            if ann[i][j] == 'L' && ann[j][i] != 'W' {
                println!("{}", "incorrect");
                return;
            }
        }
    }
    println!("{}", "correct");
}