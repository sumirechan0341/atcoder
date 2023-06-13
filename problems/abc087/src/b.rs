use std::collections::{HashMap};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        x: i32
    };
    let mut dict = HashMap::<i32, i32>::new();
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                match dict.get_mut(&(500*i + 100*j + 50 * k)) {
                    Some(x) => {
                        *x += 1;
                    },
                    None => {
                        dict.insert(500*i + 100*j + 50 * k, 1);
                    }
                }
            }
        }
    }
    println!("{}", dict.get(&x).unwrap_or(&0));
}