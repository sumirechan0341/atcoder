use itertools::Itertools;
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        sn: [Chars; n]
    };
    let mut used = vec![vec![false; m]; n];
    let mut pass = vec![vec![false; m]; n];
    dfs(&sn, (1, 1), &mut used, &mut pass);
    let mut count = 0;
    for i in 0..n {
        for j in 0..m {
            if pass[i][j] {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
fn dfs(map: &Vec<Vec<char>>, now: (usize, usize), used: &mut Vec<Vec<bool>>, pass: &mut Vec<Vec<bool>>) {

    if used[now.0][now.1] {
        return;
    }

    used[now.0][now.1] = true;
    // 下
    for i in 0.. {
        if map[now.0+i][now.1] == '.' {
            pass[now.0+i][now.1] = true;
        } else {
            dfs(map, (now.0+i-1, now.1), used, pass);
            break;
        }
    }
    // 上
    for i in 0.. {
        if map[now.0-i][now.1] == '.' {
            pass[now.0-i][now.1] = true;
        } else {
            dfs(map, (now.0-i+1, now.1), used, pass);
            break;
        }
    }
    // 右
    for i in 0.. {
        if map[now.0][now.1+i] == '.' {
            pass[now.0][now.1+i] = true;
        } else {
            dfs(map, (now.0, now.1+i-1), used, pass);
            break;
        }
    }
    // 左
    for i in 0.. {
        if map[now.0][now.1-i] == '.' {
            pass[now.0][now.1-i] = true;
        } else {
            dfs(map, (now.0, now.1-i+1), used, pass);
            break;
        }
    }
}