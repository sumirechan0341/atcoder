use proconio::{input, marker::Chars};
use itertools::Itertools;
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
    };
    let mut pcfn = vec![];
    for i in 0..n {
        input!{
            p: usize,
            c: usize,
            fc: [usize; c]
        };
        pcfn.push((p, c, fc));
    }
    for c in (0..n).permutations(2) {
        let a1 = &pcfn[c[0]];
        let a2 = &pcfn[c[1]];
        // a1が優れている
        if a1.0 <= a2.0 && (a2.2.iter().all(|x| a1.2.contains(x))) && (a1.0 < a2.0 || a1.2.iter().any(|x| !a2.2.contains(x))) {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}