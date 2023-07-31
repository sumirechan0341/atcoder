use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        h: usize,
        w: usize,
        shw: [Chars; h]
    };
    let ds = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    for i in 0..h {
        for j in 0..w {
            if shw[i][j] == '#' {
                let mut ok = false;
                for (dy, dx) in &ds {
                    if i as i32+dy < 0 || i as i32+dy >= h as i32 || j as i32+dx < 0 || j as i32+dx >= w as i32 {
                        continue;
                    }
                    if shw[(i as i32+*dy) as usize][(j as i32+*dx) as usize] == '#' {
                        ok = true;
                        break;
                    } 
                }
                if !ok {
                    println!("{}", "No");
                    return;
                }
            }
        }
    }
    println!("{}", "Yes");
}