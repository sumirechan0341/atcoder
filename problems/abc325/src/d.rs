use std::collections::{HashMap, BinaryHeap};
use core::cmp::Reverse;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut tdn: [(i128, i128); n]
    };
    tdn.sort_by(|a, b| a.0.cmp(&b.0).then((a.0+a.1).cmp(&(b.0+b.1))));
    let mut time = 0;
    let mut cursor = 0;
    let mut priority_queue = BinaryHeap::new();
    let mut ans = 0;
    loop {
        // 現在の時刻で実行できるものがないとき、次の時刻まで進める
        if priority_queue.is_empty() {
            if cursor == n {
                break;
            }
            time = tdn[cursor].0;
        }
        // 現在の時刻で実行可能なものをすべてつめる
        while cursor < n && tdn[cursor].0 <= time {
            priority_queue.push(Reverse(tdn[cursor].0+tdn[cursor].1));
            cursor += 1;
        }
        // 現在の時刻で実行不可能なものをすべて取り除く
        while !priority_queue.is_empty() && priority_queue.iter().nth(0).unwrap().0 < time {
            priority_queue.pop();
        }
        if let Some(x) = priority_queue.pop() {
            ans += 1;
        }
        time += 1;

    }
    println!("{}", ans);
}