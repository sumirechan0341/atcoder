use std::collections::VecDeque;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
        w: usize,
        chw: [Chars; h]
    };
    let mut dist = vec![vec![-1; w]; h];
    let mut queue = VecDeque::new();
    dist[0][0] = 1;
    queue.push_back((0, 0));
    while let Some((cy, cx)) = queue.pop_front() {
        if cy != h - 1 && chw[cy + 1][cx] != '#' && dist[cy + 1][cx] == -1 {
            dist[cy + 1][cx] = dist[cy][cx] + 1;
            queue.push_back((cy + 1, cx));
        }
        if cx != w - 1 && chw[cy][cx + 1] != '#' && dist[cy][cx + 1] == -1 {
            dist[cy][cx + 1] = dist[cy][cx] + 1;
            queue.push_back((cy, cx + 1));
        }
    }
    println!(
        "{}",
        dist.iter().map(|x| x.iter().max().unwrap()).max().unwrap()
    );
}
