use std::collections::VecDeque;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        h: usize,
        w: usize,
        sh: [Chars; h]
    };
    let mut queue = VecDeque::<(usize, usize)>::new();
    let mut used = vec![vec![false; w]; h];
    queue.push_back((h-1, w-1));
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if current.0 == 0 && current.1 == 0 {
            println!("{}", "Yes");
            return;
        }
        if used[current.0][current.1] {
            continue;
        }
        used[current.0][current.1] = true;
        if current.0 == h-1 {
            //これ以上下にいけない
        } else {
            if sh[current.0+1][current.1] == prev_snuke(&sh[current.0][current.1]) {
                if !used[current.0+1][current.1] {
                    queue.push_back((current.0+1, current.1))
                }
            }
        }
        if current.0 == 0 {
            // これ以上上にいけない
        } else {
            if sh[current.0-1][current.1] == prev_snuke(&sh[current.0][current.1]) {
                if !used[current.0-1][current.1] {
                    queue.push_back((current.0-1, current.1))
                }
            }
        }
        if current.1 == w-1 {
            // これ以上右にいけない
        } else {
            if sh[current.0][current.1+1] == prev_snuke(&sh[current.0][current.1]) {
                if !used[current.0][current.1+1] {
                    queue.push_back((current.0, current.1+1))
                }
            }
        }
        if current.1 == 0 {
            // これ以上左にいけない
        } else {
            if sh[current.0][current.1-1] == prev_snuke(&sh[current.0][current.1]) {
                if !used[current.0][current.1-1] {
                    queue.push_back((current.0, current.1-1))
                }
            }
        }
    }
    println!("{}", "No");
}

fn prev_snuke(c: &char) -> char {
    match c {
        's' => 'e',
        'n' => 's',
        'u' => 'n',
        'k' => 'u',
        'e' => 'k',
        _ => *c
    }
}