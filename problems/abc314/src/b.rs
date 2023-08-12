use itertools::Itertools;
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
    };
    let mut ls = vec![];
    for i in 0..n {
        input!{
            c: usize,
            ac: [i32; c]
        };
        ls.push((i+1, ac));
    }
    input!{
        x: i32,
    };
    let mut ans = ls.iter().filter(|&y| y.1.contains(&x)).collect::<Vec<_>>();
    ans.sort_by_key(|y| y.1.len());
    if ans.len() == 0 {
        println!("{}", 0);
        println!("{}", "");
    } else {
        let max = ans[0].1.len();
        let mut ans2 = ans.iter().filter(|y| y.1.len() == max).collect::<Vec<_>>();
        ans2.sort();
        println!("{}", ans2.len());
        println!("{}", ans2.iter().map(|y| y.0.to_string()).join(" "));

    }
}