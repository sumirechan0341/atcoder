use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        snn: [Chars; n],
        tnn: [Chars; n]
    };
    let mut next_snn = snn;
    let new_tnn = normalize(&tnn, n, n);
    for i in 0..4 {
        let new_snn = normalize(&next_snn, n, n);
        if is_same_matrix(&new_snn, &new_tnn, n, n) {
            println!("{}", "Yes");
            return;
        }
        next_snn = rotate(&next_snn, n, n);
    }
    println!("{}", "No");
}

fn normalize(a: &Vec<Vec<char>>, h: usize, w: usize) -> Vec<Vec<char>> {
    let mut res = vec![vec!['.'; w]; h];
    let mut min_w = w;
    let mut min_h = h;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                if i < min_h {
                    min_h = i;
                }
                if j < min_w {
                    min_w = j;
                }
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                res[i-min_h][j-min_w] = '#';
            }
        }
    }
    return res;
}
fn rotate(a: &Vec<Vec<char>>, h: usize, w: usize) -> Vec<Vec<char>> {
    let mut res = vec![vec!['.'; w]; h];
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                res[j][h-i-1] = '#';
            }
        }
    }
    return res;
}

fn is_same_matrix(a: &Vec<Vec<char>>, b: &Vec<Vec<char>>, h: usize, w: usize) -> bool {
    for i in 0..h {
        for j in 0..w {
            if a[i][j] != b[i][j] {
                return false;
            }
        }
    }
    return true;
}