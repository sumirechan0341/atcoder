use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        s: Chars,
        mut t: Chars
    };
    let mut used = HashSet::new();
    let mut queue = VecDeque::new();
    let mut start = s.clone();
    start.push('.');
    start.push('.');
    queue.push_back((start.clone(), 0));

    t.push('.');
    t.push('.');
    used.insert(start);
    while let Some((x, cost)) = queue.pop_front() {
        if x == t {
            println!("{}", cost);
            return;
        }
        let mut blank = 0;
        for i in 0..n + 1 {
            if x[i] == '.' {
                blank = i;
                break;
            }
        }

        for i in 0..n + 1 {
            let mut next = x.clone();
            if x[i] != '.' && x[i + 1] != '.' {
                next[blank] = x[i];
                next[blank + 1] = x[i + 1];
                next[i] = '.';
                next[i + 1] = '.';
            }
            if !used.contains(&next) {
                used.insert(next.clone());
                queue.push_back((next, cost + 1));
            }
        }
    }
    println!("{}", -1);
}
