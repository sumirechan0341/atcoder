use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut x: usize,
        an: [usize; n]
    };
    let mut death_note = HashSet::<usize>::new();
    while true {
        if !death_note.contains(&x) {
            death_note.insert(x);
        } else {
            println!("{}", death_note.len());
            return;
        }
        x = an[x-1];
    }

}