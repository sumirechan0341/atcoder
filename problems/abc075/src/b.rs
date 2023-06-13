use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        h: usize,
        w: usize,
        sh: [Chars; h]
    };
    let mut ans = vec![vec![0; w]; h];
    let ds = vec![(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];
    for i in 0..h {
        for j in 0..w {
            if sh[i][j] == '#' {
                // 爆弾を認識するための不可能な数
                ans[i][j] = 10;
                continue;
            }
            for d in &ds {
                if (i as i32)+d.0 < 0 || (i as i32)+d.0 >= h as i32 || (j as i32)+d.1 < 0 || (j as i32)+d.1 >= w as i32 {
                    continue;
                }
                if sh[((i as i32)+d.0) as usize][((j as i32)+d.1) as usize] == '#' {
                    ans[i][j] += 1;
                }
            }
        }
    }
    for i in 0..h {
        println!("{}", ans[i].clone().into_iter().map(|x| if x == 10 { "#".to_string() } else { x.to_string() }).collect::<String>());
    }
}