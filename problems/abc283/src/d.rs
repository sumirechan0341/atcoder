use std::collections::{VecDeque, HashSet};

use proconio::{input, marker::Chars, input_interactive};

pub fn main() {
    input! {
        s: Chars
    };
    let used = &mut VecDeque::<Vec<char>>::new();
    used.push_back(vec![]);
    for i in 0..s.len() {
        if s[i] == '(' {
            used.push_back(used.iter().last().unwrap().clone());
        } else if s[i] == ')' {
            used.pop_back();
        } else {
            let target = used.into_iter().last().unwrap();
            if target.contains(&s[i]) {
                println!("{}", "No");
                return;
            }
            target.push(s[i]);
        }
    }
    println!("{}", "Yes");
}