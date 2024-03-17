use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Chars};
pub fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize,
        abn: [(usize, usize); n]
    };
    for i in 0..1 << 2 * n {
        let mut ii = i;
        let mut v = vec![];
        let mut count = 0;
        for j in 0..2 * n {
            if ii & 1 == 1 {
                // 使う
                if j < n {
                    v.push(j);
                    count += abn[j].0 * abn[j].1;
                } else if j >= n && !v.contains(&(j - n)) {
                    v.push(j);
                    count += abn[j - n].0 * abn[j - n].1;
                }
            }
            ii = ii >> 1;
        }
        assert!(v.len() <= n);
        if count != h * w {
            continue;
        }
        let mut start = (0, 0);
        for vv in v.iter().permutations(v.len()) {
            let mut field = vec![vec!['.'; w]; h];
            let mut ok = true;
            'outer: for &v in vv {
                // 左上を毎回求める
                'start: for k in 0..h {
                    for l in 0..w {
                        if field[k][l] == '.' {
                            start = (k, l);
                            break 'start;
                        }
                    }
                }
                // k縦
                if v >= n {
                    for k in 0..abn[v - n].0 {
                        // l横
                        for l in 0..abn[v - n].1 {
                            if start.0 + k >= h || start.1 + l >= w {
                                ok = false;
                                break 'outer;
                            }
                            if field[start.0 + k][start.1 + l] == '#' {
                                ok = false;
                                break 'outer;
                            }
                            field[start.0 + k][start.1 + l] = '#';
                        }
                    }
                } else {
                    for k in 0..abn[v].1 {
                        for l in 0..abn[v].0 {
                            if start.0 + k >= h || start.1 + l >= w {
                                ok = false;
                                break 'outer;
                            }
                            if field[start.0 + k][start.1 + l] == '#' {
                                ok = false;
                                break 'outer;
                            }
                            field[start.0 + k][start.1 + l] = '#';
                        }
                    }
                }
            }
            if ok {
                let mut local = true;
                for k in 0..h {
                    for l in 0..w {
                        if field[k][l] == '.' {
                            local = false;
                        }
                    }
                }
                if local {
                    println!("{}", "Yes");
                    return;
                }
            }
        }
    }
    println!("{}", "No");
}
