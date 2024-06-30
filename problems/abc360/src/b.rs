use std::collections::{HashSet, VecDeque};

use num::Integer;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars,
        t: Chars
    };
    if s == t {
        println!("{}", "Yes");
        return;
    }
    for i in 1..s.len() {
        // 何文字で区切るか
        for j in 0..i {
            // どこ始まりか
            let mut ss = vec![];
            for k in (j..s.len()).step_by(i) {
                ss.push(s[k]);
            }
            // println!("({}, {}){:?}", i, j, ss);
            if ss == t {
                // println!("{:?}", (i, j));
                println!("{}", "Yes");
                return;
            }
        }
    }
    println!("{}", "No");
}
