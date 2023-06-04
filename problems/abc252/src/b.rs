use proconio::{input, marker::Chars};
use itertools::{izip, Itertools};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        k: usize,
        mut an: [usize; n],
        bk: [usize; k]
    };
    let mut oisi = vec![];
    let mut ian = izip!(an, 0..).collect::<Vec<_>>();
    ian.sort_by_key(|&(a, _)| a);
    for i in 0..n {
        if ian[n-1-i].0 == ian[n-1].0 {
            oisi.push(ian[n-1-i].1+1);
        } else {
            break;
        }
    }
    for o in oisi {
        if bk.contains(&o) {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}