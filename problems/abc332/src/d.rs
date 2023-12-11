use itertools::Itertools;
use proconio::{input, marker::Chars};
pub fn main() {
    input! {
        h: usize,
        w: usize,
        sahw: [i32; w*h],
        sbhw: [i32; w*h],
    };
    let mut ahw = vec![vec![0; w]; h];
    let mut bhw = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            ahw[i][j] = sahw[i * w + j];
        }
    }
    for i in 0..h {
        for j in 0..w {
            bhw[i][j] = sbhw[i * w + j];
        }
    }
    let mut ans = 1 << 30;
    let mut ahwc = ahw.clone();
    for p in (0..h).permutations(h) {
        for q in (0..w).permutations(w) {
            for i in 0..h {
                for j in 0..w {
                    ahwc[i][j] = ahw[p[i]][q[j]];
                }
            }
            if is_same_matrix(&ahwc, &bhw, h, w) {
                let mut local = 0;
                for k in 0..h {
                    for l in 0..h - k {
                        if p[k] > p[k + l] {
                            local += 1;
                        }
                    }
                }
                for k in 0..w {
                    for l in 0..w - k {
                        if q[k] > q[k + l] {
                            local += 1;
                        }
                    }
                }
                if local < ans {
                    ans = local;
                }
            }
        }
    }
    if ans == 1 << 30 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
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
fn transpose(s: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = vec![vec![0; s.len()]; s[0].len()];
    for i in 0..s[0].len() {
        for j in 0..s.len() {
            res[i][j] = s[j][i];
        }
    }
    return res;
}
