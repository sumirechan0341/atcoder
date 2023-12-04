use std::collections::VecDeque;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i64,
        n: i64
    };
    let mut used = vec![false; 1000000];
    let mut queue = VecDeque::new();
    queue.push_back((1, 0));
    while let Some((y, cost)) = queue.pop_front() {
        if y * a == n {
            println!("{}", cost + 1);
            return;
        }
        if y * a <= 999999 && !used[(y * a) as usize] {
            used[(y * a) as usize] = true;
            queue.push_back((y * a, cost + 1));
        }
        if y > 10 && y % 10 != 0 && y <= 999999 {
            let mut c = y.to_string().chars().collect::<Vec<_>>();
            let head = c.pop().unwrap();
            c.reverse();
            c.push(head);
            c.reverse();
            let next = c.iter().collect::<String>().parse::<i64>().unwrap();
            if next == n {
                println!("{}", cost + 1);
                return;
            }
            if !used[next as usize] {
                used[next as usize] = true;
                queue.push_back((next, cost + 1));
            }
        }
    }
    println!("{}", "-1");
}
