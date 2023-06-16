use itertools::Itertools;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    let mut count = 1;
    let mut ans = vec![];
    for i in 0..s.len() {
        if i == s.len()-1 {
            ans.push(vec![s[i].to_string(), count.to_string()]);
            break;
        }
        if s[i+1] == s[i] {
            count += 1;
        } else {
            ans.push(vec![s[i].to_string(), count.to_string()]);
            count = 1;
        }
    }
    println!("{}", ans.iter().flatten().join(""));
}