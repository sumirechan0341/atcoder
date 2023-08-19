use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        d: usize,
        k: usize,
        lrd: [(usize, usize); d],
        stk: [(usize, usize); k]
    };
    for &(s, t) in &stk {
        let mut left = s;
        let mut right = s;
        for i in 0..n {
            let l = lrd[i].0;
            let r = lrd[i].1;
            if left > l && left <= r {
                left = l;
            }
            if right < r && right >= l {
                right = r;
            }
            if left <= t && t <= right {
                println!("{}", i+1);
                break;
            }
        }
    }
    
}