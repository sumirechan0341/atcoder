use std::collections::HashSet;
use std::iter::FromIterator;
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        s: Chars
    };
    let mut ans = 0;
    for i in 0..n {
        let pre = HashSet::<&char>::from_iter(&s[..i]);
        let post = HashSet::<&char>::from_iter(&s[i..]);
        let mut count = 0;
        for p in pre.into_iter() {
            if post.contains(p) {
                count += 1;
            }
        }
        if ans < count {
            ans = count
        }
    }
    println!("{}", ans);
}