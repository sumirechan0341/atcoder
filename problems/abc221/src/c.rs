use proconio::{input, marker::Chars};
use itertools::Itertools;
type VS = Vec<String>;

pub fn main() {
    input!{
        n: Chars
    };
    let mut max = 0;
    for p in (0..n.len()).permutations(n.len()) {
        let mut newn = vec![];
        for i in p {
            newn.push(n[i]);
        }
        if newn[0] == '0' {
            continue;
        }
        for i in 1..=n.len()-1 {
            if newn[i] == '0' {
                continue;
            }
            let pre = newn[..i].into_iter().collect::<String>();
            let post = newn[i..].into_iter().collect::<String>();
            if max < pre.parse::<i64>().unwrap() * post.parse::<i64>().unwrap() {
                max = pre.parse::<i64>().unwrap() * post.parse::<i64>().unwrap();
            }
        }
    }
    println!("{}", max);
}