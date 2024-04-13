use std::collections::VecDeque;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [Chars; h],
        n: usize,
        rcen: [(usize, usize, usize); n]
    };
    let mut max_e = vec![vec![0; w]; h];
    for (r, c, e) in rcen {
        max_e[r - 1][c - 1] = e;
    }
    let mut now = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if ahw[i][j] == 'S' {
                now = (i, j);
            }
        }
    }
    let mut queue = VecDeque::new();
    if max_e[now.0][now.1] == 0 {
        println!("{}", "No");
        return;
    }
    queue.push_back((now, max_e[now.0][now.1]));
    let mut visited = vec![vec![false; w]; h];
    visited[now.0][now.1] = true;
    let ds = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    while let Some(((r, c), e)) = queue.pop_front() {
        if e == 0 {
            continue;
        }
        for &(dx, dy) in &ds {
            let (nr, nc) = (r as i32 + dx, c as i32 + dy);
            if nr < 0
                || h as i32 <= nr
                || nc < 0
                || w as i32 <= nc
                || ahw[nr as usize][nc as usize] == '#'
            {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if ahw[nr][nc] == 'T' {
                println!("{}", "Yes");
                return;
            }
            if visited[nr][nc] && e - 1 <= max_e[nr][nc] {
                continue;
            }
            if max_e[nr][nc] < e - 1 {
                max_e[nr][nc] = e - 1;
            }
            visited[nr][nc] = true;
            queue.push_back(((nr, nc), max_e[nr][nc]));
        }
    }
    println!("{}", "No");
}
