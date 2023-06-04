use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        n: usize,
        mut x: usize,
        mut y: usize,
        mut z: usize,
        an: [i32; n],
        bn: [i32; n]
    };
    let mut passed = vec![];
    let mut sortedx = an.iter().enumerate().collect::<Vec<(usize, &i32)>>();
    sortedx.sort_by(|a, b| b.1.cmp(a.1));
    for i in 0..n {
        if x == 0 {
            break;
        }
        if n > 0 {
            passed.push(sortedx[i].0);
            x -= 1;
        }
    }
   
    let mut sortedy = bn.iter().enumerate().collect::<Vec<(usize, &i32)>>();
    sortedy.sort_by(|a, b| b.1.cmp(a.1));
    for i in 0..n {
        if y == 0 {
            break;
        }
        if n > 0 && !passed.contains(&sortedy[i].0) {
            passed.push(sortedy[i].0);
            y -= 1;
        }
    }
   
    let mut zn = vec![0; n];
    for i in 0..n {
        zn[i] = an[i] + bn[i];
    }
    let mut sortedz = zn.iter().enumerate().collect::<Vec<(usize, &i32)>>();
    sortedz.sort_by(|a, b| b.1.cmp(a.1));
    for i in 0..n {
        if z == 0 {
            break;
        }
        if n > 0  && !passed.contains(&sortedz[i].0) {
            passed.push(sortedz[i].0);
            z -= 1;
        }
    }
    passed.sort();
    for p in passed {
        println!("{}", p + 1);
    }
}   