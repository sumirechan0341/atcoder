use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        abm: [(usize, usize); m]
    };
    let mut ans = vec![true; n];
    for (a, b) in abm {
        ans[b-1] = false;
    }
    if ans.iter().filter(|&x| *x).count() == 1 {
        println!("{}", ans.iter().position(|x| *x).unwrap()+1);
    } else {
        println!("{}", -1);
    }
}