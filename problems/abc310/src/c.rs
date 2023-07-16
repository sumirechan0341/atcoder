use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        sn: [Chars; n]
    };
    let mut set = HashSet::<Vec<char>>::new();
    let mut eq = 0;
    for mut s in sn {
        let cps = s.clone();
        s.reverse();
        if cps == s && !set.contains(&s) {
            eq += 1;
        }
        set.insert(cps);
        set.insert(s);
        
     }
     println!("{}", (set.len() + eq) /2);
}