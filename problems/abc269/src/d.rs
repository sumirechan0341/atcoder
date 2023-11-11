use std::collections::{VecDeque, HashSet};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        xyn: [(i32, i32); n]
    };
    let mut used = vec![vec![false; 2001]; 2001];
    let mut graph = HashSet::new();
    for i in 0..n {
        graph.insert(((xyn[i].0+1000) as usize, (xyn[i].1+1000) as usize));
    }
    let ds = vec![(!0, !0), (!0, 0), (0, !0), (0, 1), (1, 0), (1, 1)];
    let mut ans = 0;
    for i in 0..n {
        let mut queue = VecDeque::new();
        let (sx, sy) = xyn[i];
        if used[(sx+1000) as usize][(sy+1000) as usize] {
            continue;
        }
        queue.push_back(((sx+1000) as usize, (sy+1000) as usize));
        used[(sx+1000) as usize][(sy+1000) as usize] = true;
        while let Some((cx, cy)) = queue.pop_front() {
            for &(dx, dy) in &ds {
                let nx = cx.wrapping_add(dx);
                let ny = cy.wrapping_add(dy);
                if nx > 2000 || ny > 2000 {
                    continue;
                }
                if used[nx][ny] {
                    continue;
                }
                if graph.contains(&(nx, ny)) {
                    queue.push_back((nx, ny));
                    used[nx][ny] = true;
                }
            }
        }
        ans += 1;
    }
    println!("{}", ans);
}