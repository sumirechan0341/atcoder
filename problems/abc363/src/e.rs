use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use proconio::input;

pub fn main() {
    input! {
        h: usize,
        w: usize,
        y: i32,
        ahw: [[i32; w]; h]
    }
    let mut heap = BinaryHeap::new();

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut ans = h * w;
    let mut visited = vec![vec![false; w]; h];

    for j in 1..h - 1 {
        if !visited[j][0] {
            heap.push(Reverse((ahw[j][0], (j, 0))));
            visited[j][0] = true;
        }
        if !visited[j][w - 1] {
            heap.push(Reverse((ahw[j][w - 1], (j, w - 1))));
            visited[j][w - 1] = true;
        }
    }
    for j in 1..w - 1 {
        if !visited[0][j] {
            heap.push(Reverse((ahw[0][j], (0, j))));
            visited[0][j] = true;
        }
        if !visited[h - 1][j] {
            heap.push(Reverse((ahw[h - 1][j], (h - 1, j))));
            visited[h - 1][j] = true;
        }
    }
    if !visited[0][0] {
        heap.push(Reverse((ahw[0][0], (0, 0))));
        visited[0][0] = true;
    }
    if !visited[0][w - 1] {
        heap.push(Reverse((ahw[0][w - 1], (0, w - 1))));
        visited[0][w - 1] = true;
    }
    if !visited[h - 1][0] {
        heap.push(Reverse((ahw[h - 1][0], (h - 1, 0))));
        visited[h - 1][0] = true;
    }
    if !visited[h - 1][w - 1] {
        heap.push(Reverse((ahw[h - 1][w - 1], (h - 1, w - 1))));
        visited[h - 1][w - 1] = true;
    }
    for i in 1..=y {
        while let Some(Reverse((level, (y, x)))) = heap.pop() {
            if level > i {
                heap.push(Reverse((level, (y, x))));
                break;
            }
            ans -= 1;
            for &(dy, dx) in &directions {
                let ny = y as isize + dy;
                let nx = x as isize + dx;
                if ny >= 0 && ny < h as isize && nx >= 0 && nx < w as isize {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if !visited[ny][nx] {
                        heap.push(Reverse((ahw[ny][nx], (ny, nx))));
                        visited[ny][nx] = true;
                    }
                }
            }
        }
        println!("{}", ans);
    }
}
