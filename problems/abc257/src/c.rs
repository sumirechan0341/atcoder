use std::collections::HashMap;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        s: Chars,
        wn: [u64; n]
    };
    let mut adult = vec![];
    let mut child = vec![];
    for i in 0..n {
        if s[i] == '0' {
            child.push(wn[i]*10);
        } else {
            adult.push(wn[i]*10);
        }
    }
    child.sort();
    adult.sort();
    if adult.len() == 0 {
        println!("{}", n);
        return;
    }

    let mut max: usize = 0;
    for x in &adult {
        // partition_pointは使えない
        // let adult_out = adult.partition_point(|&y| y < *x);
        // let child_out = child.partition_point(|&y| y >= *x);
        let adult_out = adult.binary_search(&(x-1)).unwrap_err();
        let child_out = child.len() - child.binary_search(&(x-1)).unwrap_err();
        if max < n - (adult_out + child_out) {
            max = n - (adult_out + child_out);
        } 

    }
    println!("{}", max);
}