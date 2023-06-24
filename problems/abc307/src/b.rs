use itertools::Itertools;
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        sn: [Chars; n]
    };
    for c in (0..n).permutations(2) {
        let news = vec![sn[c[0]].clone(), sn[c[1]].clone()].concat();
        if is_palindrome(&news) {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}

fn is_palindrome(s: &Vec<char>) -> bool {
    for i in 0..s.len()/2 {
        if s[i] != s[s.len()-1-i] {
            return false;
        }
    }
    return true;
}