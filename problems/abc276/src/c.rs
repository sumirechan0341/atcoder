use std::ops::Index;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        pn: [usize; n]
    };
    let cppn = &mut pn.clone();
    let mut min = 1;
    for i in 0..n {
        if pn[i] == min {
            min += 1;
            continue;
        }
        if is_sorted(&pn[i+1..]) {
            let target = &mut cppn[i..].to_vec();
            target.sort();
            match target.binary_search(&pn[i]) {
                Ok(x) => {
                    cppn[i] = target[x-1];
                    target.remove(x-1);
                    target.reverse();
                    for j in 0..target.len() {
                        cppn[i+j+1] = target[j];
                    }
                },
                _ => {
                    println!("{}", "unreachable");
                }
            }
            break;
        }
    }
    println!("{}", cppn.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}

fn is_sorted(v: &[usize]) -> bool {
    let mut prev = 0;
    for e in v {
        if !(*e > prev) {
            return false;
        }
        prev = *e;
    }
    return true;
}