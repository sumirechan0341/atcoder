use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut n: usize
    };
    let mut dict = HashSet::new();
    for i in 0..100 {
        for j in 0..100 {
            for k in 0..100 {
                dict.insert(f(i) + f(j) + f(k));
            }
        }
    }
    let mut d = dict.iter().collect::<Vec<_>>();
    d.sort();
    println!("{}", d[n - 1]);
}
fn f(n: usize) -> usize {
    1.to_string()
        .repeat(15)
        .chars()
        .take(n + 1)
        .collect::<String>()
        .parse()
        .unwrap()
}
