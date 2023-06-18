use std::collections::HashSet;
use std::iter::FromIterator;
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        h: usize,
        w: usize,
        ahw: [[i32; w] ;h]
    };
    let mut count = 0;
    for i in 0..2_i32.pow((h+w-2) as u32) {
        let mut now = (1, 1);
        let mut j = i;
        let mut ok = true;
        let mut pass = vec![ahw[0][0]];
        for _ in 0..h+w-2{
            if j & 1 == 1 {
                // 下
                if now.0 >= h {
                    ok = false;
                    break;
                }
                now.0 = now.0 + 1;
                pass.push(ahw[now.0-1][now.1-1]);
            } else {
                // 右
                if now.1 >= w {
                    ok =false;
                    break;
                }
                now.1 = now.1 + 1;
                pass.push(ahw[now.0-1][now.1-1]);
            }
            j = j >> 1;
        }
        if ok {
            let check = HashSet::<i32>::from_iter(pass.clone());
            if check.len() == pass.len() {
                count += 1;
            }
        } else {
            continue;
        }
    }
    println!("{}", count);
}