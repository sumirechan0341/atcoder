use std::collections::{HashSet, VecDeque};

use num::Integer;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut v1: (usize, usize, usize, usize, usize, usize),
        mut v2: (usize, usize, usize, usize, usize, usize)
    };

    if (v1.3 > v2.0 && v1.0 < v2.3) && (v1.4 > v2.1 && v1.1 < v2.4) && (v1.5 > v2.2 && v1.2 < v2.5)
    {
        println!("{}", "Yes");
        return;
    }
    println!("{}", "No");
}
