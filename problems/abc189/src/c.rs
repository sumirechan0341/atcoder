use std::collections::HashSet;
use std::iter::FromIterator;
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i64; n]
    };
    let candidate = HashSet::<i64>::from_iter(an.clone());
    let mut max = 0;
    for i in candidate.into_iter() {
        let mut prev_index = 0;
        for j in 0..n {
            if an[j] < i {
                if (j - prev_index) as i64 * i > max {
                    max = (j - prev_index) as i64 * i;
                }
                prev_index = j+1;
            }
        }
        if max < (n - prev_index) as i64 * i {
            max = (n - prev_index) as i64 * i;
        }
    }
    println!("{}", max);
}