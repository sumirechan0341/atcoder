use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
        w: usize,
        chw: [Chars; h]
    };
    let mut ans = vec![];
    let d = vec![(1, 1), (1, -1), (-1, -1), (-1, 1)];
    let mut searched = HashSet::<(usize, usize)>::new();
    for k in (1..=h.min(w)).rev() {
        let mut count = 0;
        for i in k..h-k {
            for j in k..w-k {
                // (i, j)中心の半径kのバツ印が存在するか
                if searched.contains(&(i, j)) {
                    continue;
                }
                if chw[i][j] != '#' {
                    continue;
                }
                let mut ok = true;
                for n in 1..=k as i32 {
                    for (dx, dy) in &d {
                        if chw[(i as i32+n*(*dx)) as usize][(j as i32+n*(*dy)) as usize] != '#' {
                            ok = false;
                            break;
                        }
                    }
                    if !ok {
                        break;
                    }
                }
                if ok {
                    searched.insert((i, j));
                    count += 1;
                }
            }
        }
        ans.push(count.to_string());
    }
    ans.reverse();
    println!("{}", ans.join(" "));
}