use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
        w: usize,
        ghw: [Chars; h]
    };
    let mut used = vec![vec![false; w]; h];
    let mut now = (0, 0);
    for i in 0.. {
        let (x, y) = now;
        if used[y][x] {
            println!("{}", -1);
            return;
        }
        
        used[y][x] = true;
        match ghw[y][x] {
            'R' => {
                if x == w-1 {
                    println!("{} {}", y+1, x+1);
                    return;
                }
                now = (x+1, y);
            },
            'L' => {
                if x == 0 {
                    println!("{} {}", y+1, x+1);
                    return;
                }
                now = (x-1, y);
            },
            'U' => {
                if y == 0 {
                    println!("{} {}", y+1, x+1);
                    return;
                }
                now = (x, y-1);
            },
            'D' => {
                if y == h-1 {
                    println!("{} {}", y+1, x+1);
                    return;
                }
                now = (x, y+1);
            },
            _ => {
                println!("{}", "unreachable");
            }
        }
    }
}