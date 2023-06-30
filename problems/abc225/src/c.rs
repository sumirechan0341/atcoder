use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        bnm: [[i32; m]; n]
    };
    if n == 1 {
        for i in 0..m-1 {
            if (bnm[0][i] + 1 != bnm[0][i+1]) || bnm[0][i] % 7 == 0 {
                println!("{}", "No");
                return;
            }
        }
        println!("{}", "Yes");
        return;
    } 
    if m == 1 {
        for i in 0..n-1 {
            if bnm[i][0] + 7 != bnm[i+1][0] {
                println!("{}", "No");
                return;
            }
        }
        println!("{}", "Yes");
        return;
    }
    for i in 0..n-1 {
        for j in 0..m-1 {
            if bnm[i][j] + 1 != bnm[i][j+1] || bnm[i][j] + 7 != bnm[i+1][j] {
                println!("{}", "No");
                return;
            }
            if bnm[i][j] % 7 == 0 {
                println!("{}", "No");
                return;
            }
        }
    }
    println!("{}", "Yes");
}