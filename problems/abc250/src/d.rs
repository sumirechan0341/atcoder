use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: i128
    };
    let mut count = 0;
    let mut p_table = vec![];
    let mut p_set = HashSet::<i128>::new();
    for i in 2..1000000 {
        if is_prime(i) {
            p_table.push(i);
            p_set.insert(i);
        }
    }
    
    for q in 3..1000000 {
        if !p_set.contains(&q) {
            continue;
        }
        match p_table.binary_search(&(n/(q*q*q)).min(q-1)) {
            Ok(x) => {
                count += x+1;
            },
            Err(x) => {
                count += x;
            },
        }
    }
    println!("{}", count);
}

fn is_prime(n: i128) -> bool {
    if n < 6 {
        return n == 2 || n == 3 || n == 5;
    } else if n%6 != 1 && n%6 != 5 {
        return false;
    } else {
        for i in 2.. {
            if i*i > n {
                break;
            }
            if n%i == 0 {
                return false;
            }
        }
        return true;
    }
}