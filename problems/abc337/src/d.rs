use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        sh: [Chars; h]
    };
    let mut tate = vec![vec![0; h + 1]; w];
    let mut yoko = vec![vec![0; w + 1]; h];
    let mut tate_maru = vec![vec![0; h + 1]; w];
    let mut yoko_maru = vec![vec![0; w + 1]; h];
    for i in 0..h {
        for j in 0..w {
            if sh[i][j] == '.' {
                yoko[i][j + 1] = yoko[i][j] + 1;
                yoko_maru[i][j + 1] = yoko_maru[i][j]
            } else if sh[i][j] == 'o' {
                yoko[i][j + 1] = yoko[i][j] + 1;
                yoko_maru[i][j + 1] = yoko_maru[i][j] + 1;
            } else {
                yoko[i][j + 1] = 0;
            }
        }
    }

    for j in 0..h {
        for i in 0..w {
            if sh[j][i] == '.' {
                tate[i][j + 1] = tate[i][j] + 1;
                tate_maru[i][j + 1] = tate_maru[i][j]
            } else if sh[j][i] == 'o' {
                tate[i][j + 1] = tate[i][j] + 1;
                tate_maru[i][j + 1] = tate_maru[i][j] + 1;
            } else {
                tate[i][j + 1] = 0;
            }
        }
    }
    let mut ans = !0;
    for i in 0..h {
        for j in 0..w + 1 {
            if yoko[i][j] >= k {
                if k > j {
                    continue;
                }
                let local = yoko_maru[i][j] - yoko_maru[i][j - k];
                ans = ans.min(if local >= k { 0 } else { k - local });
            }
        }
    }
    for i in 0..w {
        for j in 0..h + 1 {
            if tate[i][j] >= k {
                if k > j {
                    continue;
                }
                let local = tate_maru[i][j] - tate_maru[i][j - k];
                ans = ans.min(if local >= k { 0 } else { k - local });
            }
        }
    }

    println!("{}", if ans == !0 { -1 } else { ans as i32 });
}
