use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        a33: [[i32; 3]; 3],
        n: usize,
        bn: [i32; n]
    };
    let mut hole = vec![vec![false; 3]; 3];
    for b in bn {
        for i in 0..3 {
            for j in 0..3 {
                if a33[i][j] == b {
                    hole[i][j] = true;
                }
            }
        }
    }
    let mut flag3 = true;
    let mut flag4 = true;
    for i in 0..3 {
        let mut flag1 = true;
        let mut flag2 = true;
        for j in 0..3 {
            if !hole[i][j] {
                flag1 = false;
            }
            if !hole[j][i] {
                flag2 = false;
            }
        }
        if !hole[i][i] {
            flag3 = false;
        }

        if  !hole[i][2-i] {
            flag4 = false
        }
        if flag1 || flag2 {
            println!("{}", "Yes");
            return;
        }
    }
    if flag3 || flag4 {
        println!("{}", "Yes");
        return;
    }
    println!("{}", "No");
    
}