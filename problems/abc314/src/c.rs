use std::collections::{HashSet, BTreeSet};

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        s: Chars,
        cn: [usize; n]
    };
    let mut colors = vec![vec![]; m+1];
    for i in 0..n {
        colors[cn[i]].push(i+1);
    }
    let mut new_colors = vec![vec![]; m+1];
    for i in 0..=m {
        let mut new_c = vec![];
        for j in 0..colors[i].len() {
            new_c.push(colors[i][(j+colors[i].len()-1)%colors[i].len()]);
        }
        new_c.reverse();
        new_colors[i] = new_c;
    }
    for i in 0..n {
        print!("{}", s[new_colors[cn[i]].pop().unwrap() as usize -1]);
    }
    println!("{}", "");
}