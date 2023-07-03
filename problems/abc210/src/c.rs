use std::collections::{HashSet, BTreeSet, HashMap};

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        k: usize,
        cn: [i32; n]
    };
    let mut map = HashMap::<i32, i32>::new();
    for i in 0..k {
        map.entry(cn[i]).and_modify(|x| *x+=1).or_insert(1);
    }
    let mut max = map.len();
    // map使ってて0になったら消すパターンの処理方法
    for i in 1..n-k+1 {
        let mut remove_flag = false;
        match map.get_mut(&cn[i-1]) {
            Some(x) => {
                if *x == 1 {
                    remove_flag = true;
                } else {
                    *x -= 1;
                }
            },
            None => {
                println!("{}", "unreachble");
            }
        }
        if remove_flag {
            map.remove(&cn[i-1]);
        }
        map.entry(cn[i+k-1]).and_modify(|x| *x+=1).or_insert(1);
        if map.len() > max {
            max = map.len();
        }
    }
    println!("{}", max);
}