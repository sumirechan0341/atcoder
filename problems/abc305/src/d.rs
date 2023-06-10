use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        an: [usize; n],
        q: usize,
        lrq: [(usize, usize); q] 
    };
    let mut sleep_sum = vec![0];

    let mut current = 0;
    for i in 0..n-1 {
        if i % 2 == 1 {
            sleep_sum.push(an[i+1] - an[i] + current);
            current += an[i+1] - an[i];
        }
    }
    for lr in lrq {
        let mut left_sleep = 0;
        match an.binary_search(&lr.0) {
            Ok(x) => {
                left_sleep += sleep_sum[x / 2];
            },
            Err(x) => {
                // 今まで寝た量
                left_sleep += sleep_sum[(x-1)/2];
                if x % 2 == 0 {
                    // 今も寝ている
                    left_sleep += lr.0 - an[x-1];
                }
            }
        }
        let mut right_sleep = 0;
        match an.binary_search(&lr.1) {
            Ok(x) => {
                right_sleep += sleep_sum[x / 2];
            },
            Err(x) => {
                // 今まで寝た量
                right_sleep += sleep_sum[(x-1)/2];
                if x % 2 == 0 {
                    // 今も寝ている
                    right_sleep += lr.1 - an[x-1];
                }
            }
        }

        println!("{}", right_sleep - left_sleep);
    }
}