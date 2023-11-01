use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i64,
        m: i64
    };
    let mut left = 0;
    let mut right = 1000;
    let mut sq_dict = vec![];
    while left < m {
        if left > right {
            break;
        }
        while left*left + right*right > m {
            right -= 1;
            if left > right {
                break;
            }
        }
        if left > right {
            break;
        }
        if left*left + right*right == m {
            sq_dict.push((left, right));
        }
        left += 1;
    }
    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), 0));
    let mut ans = vec![vec![-1; n as usize]; n as usize];
    ans[0][0] = 0;
    while let Some(((cx, cy), cost)) = queue.pop_front() {                
        for &(x, y) in &sq_dict {
            let nx = cx+x as usize;
            let ny = cy+y as usize;
            if nx < n as usize && ny < n as usize {
                if ans[nx][ny] == -1 {
                    ans[nx][ny] = cost+1;
                    queue.push_back(((nx, ny), cost+1));
                }
            }

            let nx2 = cx.wrapping_sub(x as usize);
            let ny2 = cy+y as usize;
            if nx2 < n as usize && ny2 < n as usize {
                if ans[nx2][ny2] == -1 {
                    ans[nx2][ny2] = cost+1;
                    queue.push_back(((nx2, ny2), cost+1));
                }
            }

            let nx3 = cx+x as usize;
            let ny3 = cy.wrapping_sub(y as usize);
            if nx3 < n as usize && ny3 < n as usize {
                if ans[nx3][ny3] == -1 {
                    ans[nx3][ny3] = cost+1;
                    queue.push_back(((nx3, ny3), cost+1));
                }
            }

            let nx4 = cx.wrapping_sub(x as usize);
            let ny4 = cy.wrapping_sub(y as usize);
            if nx4 < n as usize && ny4 < n as usize {
                if ans[nx4][ny4] == -1 {
                    ans[nx4][ny4] = cost+1;
                    queue.push_back(((nx4, ny4), cost+1));
                }
            }

            let nx5 = cx+y as usize;
            let ny5 = cy+x as usize;
            if nx5 < n as usize && ny5 < n as usize {
                if ans[nx5][ny5] == -1 {
                    ans[nx5][ny5] = cost+1;
                    queue.push_back(((nx5, ny5), cost+1));
                }
            }

            let nx6 = cx.wrapping_sub(y as usize);
            let ny6 = cy+x as usize;
            if nx6 < n as usize && ny6 < n as usize {
                if ans[nx6][ny6] == -1 {
                    ans[nx6][ny6] = cost+1;
                    queue.push_back(((nx6, ny6), cost+1));
                }
            }

            let nx7 = cx+y as usize;
            let ny7 = cy.wrapping_sub(x as usize);
            if nx7 < n as usize && ny7 < n as usize {
                if ans[nx7][ny7] == -1 {
                    ans[nx7][ny7] = cost+1;
                    queue.push_back(((nx7, ny7), cost+1));
                }
            }

            let nx8 = cx.wrapping_sub(y as usize);
            let ny8 = cy.wrapping_sub(x as usize);
            if nx8 < n as usize && ny8 < n as usize {
                if ans[nx8][ny8] == -1 {
                    ans[nx8][ny8] = cost+1;
                    queue.push_back(((nx8, ny8), cost+1));
                }
            }
        }
    }
    for i in 0..n as usize {
        println!("{}", ans[i].iter().map(|x| x.to_string()).join(" "));
    }
}