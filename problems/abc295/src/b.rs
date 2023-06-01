use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        r: usize,
        c: usize,
        mut b: [Chars; r]
    };
    for i in 0..r {
        for j in 0..c {
            match b[i][j].to_digit(10) {
                None => {},
                Some(n) => {
                    b[i][j] = '.';
                    let m = n as usize;
                    let mut up_search = true;
                    let mut down_search = true;
                    for k in 0..m+1 {
                        if i + k >= r {
                            down_search = false;
                        }
                        if i as i32 - (k as i32) < 0 {
                            up_search = false;
                        }
                        if down_search {
                            // 下方向
                            // 右方向
                            for l in 0..m-k+1 {
                                if j + l >= c {
                                    break;
                                }
                                if b[i+k][j+l] == '#' {
                                    b[i+k][j+l] = '.';
                                }
                            }
                            // 左方向
                            for l in 0..m-k+1 {
                                if j as i32 - (l as i32) < 0 {
                                    break;
                                }
                                if b[i+k][j-l] == '#' {
                                    b[i+k][j-l] = '.';
                                }
                            }
                        }
                        if up_search {
                            // 上方向
                            // 右方向
                            for l in 0..m-k+1 {
                                if j + l >= c {
                                    break;
                                }
                                if b[i-k][j+l] == '#' {
                                    b[i-k][j+l] = '.';
                                }
                            }
                            // 左方向
                            for l in 0..m-k+1 {
                                if j as i32 - (l as i32) < 0 {
                                    break;
                                }
                                if b[i-k][j-l] == '#' {
                                    b[i-k][j-l] = '.';
                                }
                            }
                        }

                    }
                }
            }
        }
    }
    for i in 0..r {
        for j in 0..c {
            print!("{}", b[i][j]);
        }
        println!("{}", "");
    }
}

fn manhattan_dist(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    return (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
}