use std::collections::HashMap;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        s: Chars,
        wn: [usize; n]
    };
    let mut adult = HashMap::<usize, usize>::new();
    let mut child = vec![];
    for i in 0..n {
        if s[i] == '0' {
            child.push(wn[i]);
        } else {
            adult.entry(wn[i]).and_modify(|x| {
                *x += 1;
            }).or_insert(1);
        }
    }
    child.sort();
    let mut keys = adult.keys().collect::<Vec<_>>();
    keys.sort();
    let mut total_adult_out = 0;
    for x in keys {
        let will_out_adult = adult.get(x).unwrap();
        let will_out_child = child.len() - child.partition_point(|y| y > x);
        println!("vec {:?}", child);
        println!("len {}", child.len());
        println!("part {}", child.partition_point(|y| y > x));
        println!("x {}", x);
        println!("{}", will_out_child);
        if *will_out_adult >= will_out_child {
            println!("{}", n-total_adult_out-will_out_child);
            return;
        }
        total_adult_out += will_out_adult;
    }
    println!("{}", n);
}