use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h1: usize,
        w1: usize,
        ahw: [[i32; w1]; h1],
        h2: usize,
        w2: usize,
        bhw: [[i32; w2]; h2]
    };
    for i in 0..2_i32.pow(h1 as u32) {
        if i.count_ones() != h2 as u32 {
            continue;
        }
        for j in 0..2_i32.pow(w1 as u32) {
            if j.count_ones() != w2 as u32 {
                continue;
            }
            let mut new_matrix = vec![];
            let mut rd = i;
            let mut cd = j;
            for k in 0..h1 {
                if rd & 1 == 1 {
                    new_matrix.push(ahw[k].clone());
                }
                rd = rd >> 1;
            }
            // 転置
            let mut tnew_matrix = vec![];
            for k in 0..w1 {
                let mut local = vec![];
                for l in 0..h2 {
                    local.push(new_matrix[l][k]);
                }
                tnew_matrix.push(local);
            }
            let mut new_matrix2 = vec![];
            for k in 0..w1 {
                if cd & 1 == 1 {
                    new_matrix2.push(tnew_matrix[k].clone());
                }
                cd = cd >> 1;
            }
            let mut ans = vec![vec![0; w2]; h2];
            for k in 0..w2 {
                for l in 0..h2 {
                    ans[l][k] = new_matrix2[k][l];
                }
            }
            
            if is_same_matrix(&ans, &bhw, h2, w2) {
                println!("{}", "Yes");
                return;
            }
        }
    }
    println!("{}", "No");
}

fn is_same_matrix(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>, h: usize, w: usize) -> bool {
    for i in 0..h {
        for j in 0..w {
            if a[i][j] != b[i][j] {
                return false;
            }
        }
    }
    return true;
}
