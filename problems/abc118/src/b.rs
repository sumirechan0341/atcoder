use std::collections::{HashSet, HashMap};

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        m: usize,
    };
    let mut ans = HashMap::<usize, usize>::new();
    for i in 0..m {
        ans.insert(i+1, 0);
    }
    for _i in 0..n {
        input! {
            k: usize,
            ak: [usize; k]
        }
        for a in ak {
            let x = ans.get_mut(&a).unwrap();
            *x += 1;
        }
    }
    println!("{}", ans.into_iter().filter(|(i, num)| num == &n).collect::<Vec<_>>().len());

}