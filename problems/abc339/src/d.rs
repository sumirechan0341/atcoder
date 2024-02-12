use std::collections::VecDeque;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        snn: [Chars; n]
    };
    let mut cost = vec![vec![vec![vec![-1; n]; n]; n]; n];
    let mut start1 = (0, 0);
    let mut start2 = (0, 0);
    let mut idx = 0;
    for i in 0..n {
        for j in 0..n {
            if snn[i][j] == 'P' {
                if idx == 0 {
                    start1 = (i, j);
                } else {
                    start2 = (i, j);
                }
                idx += 1;
            }
        }
    }
    let mut queue = VecDeque::new();
    queue.push_back((start1, start2));
    cost[start1.0][start1.1][start2.0][start2.1] = 0;
    while let Some((p1, p2)) = queue.pop_front() {
        if p1.0 == p2.0 && p1.1 == p2.1 {
            println!("{}", cost[p1.0][p1.1][p2.0][p2.1]);
            return;
        }
        let d = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        for (dy, dx) in d {
            let (mut ny1, mut nx1) = (p1.0 as i32 + dy, p1.1 as i32 + dx);
            let (mut ny2, mut nx2) = (p2.0 as i32 + dy, p2.1 as i32 + dx);
            if ny1 >= 0
                && ny1 < n as i32
                && nx1 >= 0
                && nx1 < n as i32
                && snn[ny1 as usize][nx1 as usize] != '#'
            {
            } else {
                ny1 = p1.0 as i32;
                nx1 = p1.1 as i32;
            }
            if ny2 >= 0
                && ny2 < n as i32
                && nx2 >= 0
                && nx2 < n as i32
                && snn[ny2 as usize][nx2 as usize] != '#'
            {
            } else {
                ny2 = p2.0 as i32;
                nx2 = p2.1 as i32;
            }
            if cost[ny1 as usize][nx1 as usize][ny2 as usize][nx2 as usize] == -1 {
                cost[ny1 as usize][nx1 as usize][ny2 as usize][nx2 as usize] =
                    cost[p1.0 as usize][p1.1 as usize][p2.0 as usize][p2.1 as usize] + 1;
                queue.push_back(((ny1 as usize, nx1 as usize), (ny2 as usize, nx2 as usize)));
            }
        }
    }
    println!("{}", -1);
}
