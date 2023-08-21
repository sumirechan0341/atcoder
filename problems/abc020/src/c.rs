use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        h: usize,
        w: usize,
        t: usize,
        chw: [Chars; h]
    };
    // 解説AC
    
    let mut sat = 1;
    let mut unsat = 10000000000;
    let mut start = (0, 0);
    let mut goal = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if chw[i][j] == 'S' {
                start = (i, j);
            }
            if chw[i][j] == 'G' {
                goal = (i, j);
            }
        }
    }
    while unsat-sat > 1 {
        let med = (sat+unsat)/2;
        let mut queue = VecDeque::<(usize, usize)>::new();
        let mut dist = vec![vec![100000000000; w]; h];
        dist[start.0][start.1] = 0;
        queue.push_back(start);
        let direction = vec![(!0, 0), (1, 0), (0, 1), (0, !0)];
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            for &d in &direction {
                let nx = current.0.wrapping_add(d.0);
                let ny = current.1.wrapping_add(d.1);
                if nx < h && ny < w {
                    let cost =  if chw[nx][ny] == '#' {
                        med
                    } else {
                        1
                    };
                    if dist[nx][ny] > dist[current.0][current.1] + cost {
                        dist[nx][ny] = dist[current.0][current.1] + cost;
                        queue.push_back((nx, ny));
                    }
                }   
            }
        }
        if dist[goal.0][goal.1] <= t {
            sat = med;
        } else {
            unsat = med;
        }
    }
    println!("{:?}", sat);

    
}