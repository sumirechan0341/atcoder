use std::collections::VecDeque;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [usize; n]
    };
    let mut used = vec![false; n];
    let mut ans = vec![];
    let mut next = 0;
    let mut used_index = vec![0; n];
    for i in 0 as usize.. {
        if used[next] {
            break;
        }
        ans.push((next+1).to_string());
        used[next] = true;
        used_index[next] = i;
        next = an[next]-1
    }
    println!("{}", ans.split_at(used_index[next]).1.len());
    println!("{}", ans.split_at(used_index[next]).1.join(" "));
}