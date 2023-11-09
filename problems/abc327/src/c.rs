use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a99: [[usize; 9]; 9]
    };
    for i in 0..9 {
        let mut used1 = vec![false; 10];
        let mut used2 = vec![false; 10];
        let mut used3 = vec![false; 10];
        for j in 0..9 {
            if used1[a99[i][j]] {
                println!("{}", "No");
                return;
            }
            used1[a99[i][j]] = true;
            if used2[a99[j][i]] {
                println!("{}", "No");
                return;
            }
            used2[a99[j][i]] = true;
            if used3[a99[i/3*3+j/3][(i%3)*3+j%3]] {
                println!("{}", "No");
                return;
            }
            used3[a99[i/3*3+j/3][(i%3)*3+j%3]] = true;
        }
    }
    println!("{}", "Yes");
}