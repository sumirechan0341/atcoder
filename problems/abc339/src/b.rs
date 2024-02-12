use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize
    };
    // y, x
    let d = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut ans = vec![vec!['.'; w]; h];
    // (y, x)
    let mut now: (i32, i32) = (0, 0);
    let mut d_i = 0;
    for i in 0..n {
        if ans[now.0 as usize][now.1 as usize] == '.' {
            ans[now.0 as usize][now.1 as usize] = '#';
            d_i += 1;
            d_i %= 4;
            now = (
                (now.0 + h as i32 + d[d_i].0) % h as i32,
                (now.1 + w as i32 + d[d_i].1) % w as i32,
            );
        } else {
            ans[now.0 as usize][now.1 as usize] = '.';
            d_i += 3;
            d_i %= 4;
            now = (
                (now.0 + h as i32 + d[d_i].0) % h as i32,
                (now.1 + w as i32 + d[d_i].1) % w as i32,
            );
        }
    }
    for i in 0..h {
        println!("{}", ans[i].iter().collect::<String>());
    }
}
