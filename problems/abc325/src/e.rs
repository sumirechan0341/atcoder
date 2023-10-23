use std::{collections::BinaryHeap, cmp::Reverse};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        a: i64,
        b: i64,
        c: i64,
        dnn: [[i64; n]; n]
    };
    let max = std::i64::MAX/2;
    let mut d1 = vec![max; n];
    let mut d2 = vec![max; n];
    let mut q1 = BinaryHeap::new();
    let mut q2 = BinaryHeap::new();
    q1.push((Reverse(0), 0));
    q2.push((Reverse(0), n-1));
    for i in 1..n {
        q1.push((Reverse(max), i));
        q2.push((Reverse(max), n-1-i));
    }
    d1[0] = 0;
    while let Some((Reverse(cost), now)) = q1.pop() {
        if cost > d1[now] {
            continue;
        }
        d1[now] = cost;
        for next in 0..n {
            if d1[next] > d1[now]+dnn[now][next]*a {
                d1[next] = d1[now]+dnn[now][next]*a;
                q1.push((Reverse(d1[next]), next));
            }
        }
    }
    // ゴールから逆順にたどる
    d2[n-1] = 0;
    while let Some((Reverse(cost), now)) = q2.pop() {
        if cost > d2[now] {
            continue;
        }
        d2[now] = cost;
        for next in 0..n {
            if d2[next] > d2[now]+dnn[now][next]*b+c {
                d2[next] = d2[now]+dnn[now][next]*b+c;
                q2.push((Reverse(d2[next]), next));
            }
        }
    }
    let mut min = max;
    for i in 0..n {
        if min > d1[i] + d2[i] {
            min = d1[i] + d2[i];
        }
    }
    println!("{}", min);
}