use itertools::Itertools;
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        mut s: Chars,
        q: usize,
        txcq: [(i32, usize, char); q]
    };
    let mut cp_q = txcq.clone();
    cp_q.reverse();
    let last = q - 1 - cp_q.iter().find_position(|x| x.0 != 1).map(|x| x.0).unwrap_or(0);
    for i in 0..q {
        let (t, x, c) = txcq[i];
        if t == 1 {
            s[x-1] = c;
        } else if t == 2 && i==last {
            // 大文字を小文字
            for j in 0..n {
                if s[j].is_uppercase() {
                    s[j] = s[j].to_ascii_lowercase();
                }
            }
        } else if i==last {
            // 小文字を大文字
            for j in 0..n {
                if s[j].is_lowercase() {
                    s[j] = s[j].to_ascii_uppercase();
                }
            }
        }
    }
    println!("{}", s.iter().join(""));
}