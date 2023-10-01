use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        p1: [Chars; 4],
        p2: [Chars; 4],
        p3: [Chars; 4]
    };
    let brank = vec![vec!['.'; 4]; 4];
    for rot1 in 0..4 {
        let poly1 = rotate_matrix(&p1, rot1, 4);
        let norm_poly1 = normalize(&poly1, 4, 4);
        for rot2 in 0..4 {
            let poly2 = rotate_matrix(&p2, rot2, 4);
            let norm_poly2 = normalize(&poly2, 4, 4);
            for rot3 in 0..4 {
                let poly3 = rotate_matrix(&p3, rot3, 4);
                let norm_poly3 = normalize(&poly3, 4, 4);
                 // 1

                for i in 0..4 {
                    for j in 0..4 {
                        let mut brank1 = brank.clone();
                        let mut ok1 = true;
                        'outer1: for o in 0..4 {
                            for p in 0..4 {
                                if norm_poly1[o][p] == '#' {
                                    if i+o > 3 || j+p > 3 {
                                        ok1 = false;
                                        break 'outer1;
                                    }
                                    brank1[i+o][j+p] = '#'
                                }
                            }
                        }
                        if !ok1 {
                            continue;
                        }
                        // 2
                        for k in 0..4 {
                            for l in 0..4 {
                                let mut brank2 = brank1.clone();
                                let mut ok2 = true;
                                'outer2: for o in 0..4 {
                                    for p in 0..4 {
                                        if norm_poly2[o][p] == '#' {
                                            if k+o > 3 || l+p > 3 {
                                                ok2 = false;
                                                break 'outer2;
                                            }
                                            if brank2[k+o][l+p] == '#' {
                                                ok2 = false;
                                                break 'outer2;
                                            } 
                                            brank2[k+o][l+p] = '#'
                                        }
                                    }
                                }
                                if !ok2 {
                                    continue;
                                }
                                // 3
                                for m in 0..4 {
                                    for n in 0..4 {
                                        let mut brank3 = brank2.clone();
                                        let mut ok3 = true;
                                        'outer3: for o in 0..4 {
                                            for p in 0..4 {
                                                if norm_poly3[o][p] == '#' {
                                                    if m+o > 3 || n+p > 3 {
                                                        ok3 = false;
                                                        break 'outer3;
                                                    }
                                                    if brank3[m+o][n+p] == '#' {
                                                        ok3 = false;
                                                        break 'outer3;
                                                    } 
                                                    brank3[m+o][n+p] = '#'
                                                }
                                            }
                                        }
                                        if !ok3 {
                                            continue;
                                        } else {
                                            let mut ok = true;
                                            for h in 0..4 {
                                                for w in 0..4 {
                                                    if brank3[h][w] == '.' {
                                                        ok = false;
                                                    }
                                                }
                                            }
                                            if ok {
                                                println!("{}", "Yes");
                                                return;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                                
            }
        }
    }
    println!("{}", "No");
}

fn rotate_matrix(a: &Vec<Vec<char>>, rot_num: u32, n: usize) -> Vec<Vec<char>> {
    let mut new_matrix = a.clone();
    for _r in 0..rot_num {
        let prev_matrix = new_matrix.clone();
        for i in 0..n {
            for j in 0..n {
                new_matrix[n-j-1][i] = prev_matrix[i][j];
            }
        }
    }
    return new_matrix;
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
