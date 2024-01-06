use std::collections::VecDeque;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize
    };
    let mut ans = vec![vec!["0".to_string(); n]; n];
    let d: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut now_d = 0;
    let mut now = (0, 0);
    ans[0][0] = 1.to_string();
    let mut i = 1;
    loop {
        i += 1;
        if now.0 + d[now_d].0 == n as i32
            || now.0 + d[now_d].0 < 0
            || now.1 + d[now_d].1 == n as i32
            || now.1 + d[now_d].1 < 0
            || ans[(now.0 + d[now_d].0) as usize][(now.1 + d[now_d].1) as usize] != "0"
        {
            now_d += 1;
            now_d %= 4;
            i -= 1;
        } else {
            now.0 = now.0 + d[now_d].0;
            now.1 = now.1 + d[now_d].1;
            ans[now.0 as usize][now.1 as usize] = if i == n * n {
                "T".to_string()
            } else {
                i.to_string()
            };
        }
        if i == n * n {
            break;
        }
    }
    for i in 0..n {
        println!("{}", ans[i].join(" "));
    }
}
