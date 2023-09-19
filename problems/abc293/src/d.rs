use std::collections::VecDeque;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        abcdm: [(usize, char, usize, char); m]
    };
    
    let mut graph = vec![vec![vec![]; n+1]; 2];
    for &(a, b, c, d) in &abcdm {
        graph[ctoi(b)][a].push((c, ctoi(d)));
        graph[ctoi(d)][c].push((a, ctoi(b)));
    }
    let mut used = vec![false; n+1];
    let mut x = 0;
    let mut y = 0;
    for i in 1..=n  {
        if used[i] {
            continue;
        }
        let mut queue = VecDeque::<(usize, usize)>::new();
        queue.push_back((i, 0));
        let mut exist_cycle = false;
        while let Some((current, color)) = queue.pop_back() {
            used[current] = true;
            for &(next, nx_color) in &graph[color][current] {
                if used[next] {
                    exist_cycle = true;
                    break;
                }
                // つながっていない方の色から続きを探索する
                queue.push_back((next, nx_color^1));
            }
            if exist_cycle {
                break;
            }
        }
        if exist_cycle {
            x += 1;
        } else {
            // 反対側からたどれる点全部サイクルではないので、使用済みラベルにしていく
            let mut queue = VecDeque::<(usize, usize)>::new();
            queue.push_back((i, 1));
            while let Some((current, color)) = queue.pop_back() {
                used[current] = true;
                for &(next, nx_color) in &graph[color][current] {
                    if used[next] {
                        continue;
                    }
                    // つながっていない方の色から続きを探索する
                    queue.push_back((next, nx_color^1));
                }
            }
            y += 1;
        }
    }
    println!("{} {}", x, y);
}

fn ctoi(c: char) -> usize {
    if c == 'R' {
        return 0;
    }
    if c == 'B' {
        return 1;
    }
    return !0;
}
