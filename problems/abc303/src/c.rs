use proconio::{input, marker::Chars};
use std::collections::HashSet;
pub fn main() {
    input !{
        n: usize,
        m: usize,
        mut h: i64,
        k: i64,
        s: Chars,
        xy: [[i64; 2]; m]
    };
    let mut x= 0;
    let mut y = 0;
    let mut xy_set: HashSet<(i64, i64)> = HashSet::new();
    for i in 0..m {
        xy_set.insert((xy[i][0], xy[i][1]));
    }
    for i in 0..n {
        // 移動
        if s[i] == 'R' {
            x += 1;
        }
        if s[i] == 'L' {
            x -= 1;
        }
        if s[i] == 'U' {
            y += 1;
        }
        if s[i] == 'D' {
            y -= 1;
        }
        h -= 1;
        // アイテム
        if h < 0 {
            println!("{}", "No");
            return;
        }
        if (h < k) && xy_set.contains(&(x, y)) {
            xy_set.remove(&(x, y));
            h = k;
        }
    }
    println!("{}", "Yes");
}