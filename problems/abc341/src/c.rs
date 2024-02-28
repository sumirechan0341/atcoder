use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        t: Chars,
        sh: [Chars; h]
    };
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            // y, x
            let mut now = (i, j);
            if sh[i][j] == '#' {
                continue;
            }
            let mut ok = true;
            for &c in &t {
                if c == 'L' {
                    if now.1 > 0 && sh[now.0][now.1 - 1] != '#' {
                        now.1 -= 1;
                    } else {
                        ok = false;
                        break;
                    }
                } else if c == 'R' {
                    if now.1 + 1 < w && sh[now.0][now.1 + 1] != '#' {
                        now.1 += 1;
                    } else {
                        ok = false;
                        break;
                    }
                } else if c == 'U' {
                    if now.0 > 0 && sh[now.0 - 1][now.1] != '#' {
                        now.0 -= 1;
                    } else {
                        ok = false;
                        break;
                    }
                } else {
                    if now.0 + 1 < h && sh[now.0 + 1][now.1] != '#' {
                        now.0 += 1;
                    } else {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
