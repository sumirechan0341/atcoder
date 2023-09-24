use std::collections::VecDeque;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        q: usize,
    };
    let p = 998244353;

    let mut mod_table = vec![1; q+1];
    for i in 0..q {
        mod_table[i+1] = (mod_table[i]*10)%p;
    }
    let mut now_mod = 1;
    let mut ans = vec![];
    let mut digit_series = VecDeque::<i64>::new();
    digit_series.push_back(1);
    for _i in 0..q {
        input! {
            query_id: i32
        };
        if query_id == 1 {
            input! {
                x: i64
            }
            now_mod = now_mod*10 + x;
            now_mod %= p;
            digit_series.push_back(x);
        } else if query_id == 2 {
            let d = digit_series.pop_front().unwrap();
            now_mod -= mod_table[digit_series.len()]*d;
            now_mod = (now_mod+p*10)%p 
        } else {
            ans.push(now_mod.to_string());
        }
    }
    println!("{}", ans.join("\n"));
}