use proconio::{input, marker::Chars};
use itertools::Itertools;
pub fn main() {
    input! {
        n: usize,
        pn: [usize; n],
        qn: [usize; n]
    };
    if pn == qn {
        println!("{}", 0);
        return;
    }
    let mut count_flag = false;
    let mut count = 0;
    for perm in (1..=n).permutations(n) {
        if pn == perm || qn == perm {
            if count_flag {
                println!("{}", count);
                return;
            } else {
                count_flag = true;
            }
        }
        if count_flag {
            count += 1;
        }
    }
    println!("{}", count);
}