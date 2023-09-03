use std::collections::{HashSet, HashMap};

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: Chars
    };
    // 解説AC
    let mut count_map = HashMap::<&Vec<i32>, i64>::new();
    let mut count = vec![vec![0; 10]; s.len()+1];
    for i in 0..s.len() {
        for j in 0..10 {
            if j == s[i].to_digit(10).unwrap() {
                count[i+1][j as usize] = count[i][j as usize] ^ 1;
            } else {
                count[i+1][j as usize] = count[i][j as usize];
            }
        }
    }
    for i in 0..s.len()+1 {
        count_map.entry(&count[i]).and_modify(|x| *x += 1).or_insert(1);
    }
    let mut ans = 0;
    
    for (_k, v) in count_map {
        ans += v*(v-1)/2;
    }
    println!("{}", ans);
}