use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        h: usize,
        w: usize,
        chw: [Chars; h]
    };
    // 解説AC
    let mut rows = vec![vec![0; 26]; h];
    let mut cols = vec![vec![0; 26]; w];

    let mut erased_row_total = 0;
    let mut erased_col_total = 0;

    let mut prev_erased_row_total = 0;
    let mut prev_erased_col_total = 0;

    let mut erased_row = vec![false; h];
    let mut erased_col = vec![false; w];
    for i in 0..h {
        for j in 0..w {
            rows[i][(chw[i][j] as u8-97) as usize] += 1;
        }
    }
    for j in 0..w {
        for i in 0..h {
            cols[j][(chw[i][j] as u8-97) as usize] += 1;
        }
    }
    loop {
        let mut erase_row = vec![];
        let mut erase_col = vec![];
        for i in 0..h {
            if erased_row[i] {
                continue;
            }
            for j in 0..26 {
                if w-erased_col_total > 1 && rows[i][j] == w-erased_col_total {
                    erase_row.push((i, j));
                }
            }
        }
       
        for i in 0..w {
            if erased_col[i] {
                continue;
            }
            for j in 0..26 {
                if h-erased_row_total > 1 && cols[i][j] == h-erased_row_total {
                    erase_col.push((i, j));
                }
            }
        }
        for &(r, c) in &erase_row {
            rows[r][c] = 0;
            for j in 0..w {
                if erased_col[j] {
                    continue;
                }
                cols[j][c] -= 1;
            }
            erased_row[r] = true;
            erased_row_total += 1;
        }

        for &(col, c) in &erase_col {
            cols[col][c] = 0;
            for j in 0..h {
                if erased_row[j] {
                    continue;
                }
                rows[j][c] -= 1;
            }
            erased_col[col] = true;
            erased_col_total += 1;
        }

        if erased_col_total == prev_erased_col_total && erased_row_total == prev_erased_row_total {
            break;
        }
        prev_erased_col_total = erased_col_total;
        prev_erased_row_total = erased_row_total;
    }
    println!("{}", (h-erased_row_total)*(w-erased_col_total));
}