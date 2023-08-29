use std::collections::VecDeque;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        r: usize,
        c: usize,
        syx: (usize, usize),
        gyx: (usize, usize),
        ccr: [Chars; r]
    };
    let mut queue = VecDeque::<(usize, usize)>::new();
    let ds = vec![[0, 1], [1, 0], [!0, 0], [0, !0]];
    queue.push_front((syx.0-1, syx.1-1));
    let mut used = vec![vec![-1; c]; r];
    used[syx.0-1][syx.1-1] = 0;
    while let Some(current) = queue.pop_front() {
        if current.0 == gyx.0-1 && current.1 == gyx.1-1 {
            break;
        }
        
        for &d in &ds {
            let ny = current.0.wrapping_add(d[0]);
            let nx = current.1.wrapping_add(d[1]);
            if ny < r && nx < c {
                if used[ny][nx] != -1 {
                    continue;
                }
                if ccr[ny][nx] == '#' {
                    continue;
                }
                queue.push_back((ny, nx));
                used[ny][nx] = used[current.0][current.1]+1;
            }
        }
    }
    println!("{}", used[gyx.0-1][gyx.1-1]);
}