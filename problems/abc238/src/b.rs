use std::{collections::HashSet, fmt::Debug};

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        an: [i32; n]
    };
    let mut cut_point = vec![];
    let mut rot = 0;
    cut_point.push(0);
    for a in an {
        rot = (a + rot) % 360;
        cut_point.push(rot);
    }
    cut_point.sort();
    let mut max = 0;
    for i in 0..n+1 {
        let deg = if i == n {
            360 - cut_point[i]
        } else {
            cut_point[i+1] - cut_point[i]
        };
        if max < deg {
            max = deg;
        }
    }
    println!("{}", max);
}