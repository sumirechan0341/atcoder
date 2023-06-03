use std::collections::VecDeque;
use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::collections::HashSet;
use std::collections::HashMap;

pub fn main() {
    input !{
        n: usize,
        d: f32,
        xyn: [(f32, f32); n]
    };
    let mut queue = VecDeque::<usize>::new();
    queue.push_front(0);
    let mut ans = vec![false; n];
    while !queue.is_empty() {
        let target = queue.pop_front().unwrap();
        for i in 0..n {
            if !ans[i] && distance(xyn[i], xyn[target]) <= d {
                ans[i] = true;
                queue.push_front(i);
            }
        }
    }
    for i in 0..n {
       println!("{}", if ans[i] { "Yes" } else { "No" }); 
    }
}
fn distance(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    return ((p1.0 - p2.0).powf(2.0) + (p1.1 - p2.1).powf(2.0)).sqrt();
}