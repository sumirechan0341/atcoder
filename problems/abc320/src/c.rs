use proconio::{input, marker::Chars, input_interactive};
use itertools::Itertools;
pub fn main() {
    input! {
        m: usize,
        s1: Chars,
        s2: Chars,
        s3: Chars
    };
    let ss = vec![s1, s2, s3];
    let mut min = !0;
    for perm in (0..3usize).permutations(3) {
        for i in 0..10 {
            let mut count = 0;
            let mut ng = false;
            let mut first = true;
            for &p in &perm {
                let mut ok = false;
                for j in 0..3*m {
                    if ss[p][j%m].to_digit(10).unwrap() == i {
                        if !first && j <= count {
                            continue;
                        }
                        ok = true; 
                        first = false;
                        count = j;
                        break;
                    }
                }
                if !ok {
                    ng = true;
                    break;
                }
            }
            if ng {
                continue;
            }
            if min > count {
                min = count;
            } 
        }
    }
    println!("{}", if min == !0 { -1 } else { min as i32 });
}