use proconio::{input, marker::Chars};
use itertools::Itertools;
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i32; n]
    };
    if n == 1 {
        println!("{}", an[0]);
        return;
    }
    let mut min = std::i32::MAX;
    for i in 2..=n {
        for c in (1..=n-1).combinations(i-1) {
            let mut prev = 0;
            let mut ors = vec![];
            for j in c {
                let mut or = 0;
                for k in prev..j {
                    or = or | an[k];
                }
                prev = j;
                ors.push(or);
            }
            // 末尾をつめる
            let mut or = 0;
            for k in an[prev..].iter() {
                or = or | k;
            }
            ors.push(or);

            let mut xor = 0;
            for or in ors {
                xor = xor ^ or;
            }
            if min > xor {
                min = xor;
            }
        }
    }
    println!("{}", min);
}