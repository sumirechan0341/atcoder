use std::collections::{HashSet, HashMap};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut an: [i32; n]
    };
    an.sort();
    let mut sum = 0;
    for &a in &an {
        sum ^= a;
    }
    let mut cv = HashMap::new();
    for i in 0..n {
        cv.entry(an[i]).and_modify(|x| *x += 1).or_insert(1);
    }
    if sum == 0 {
        // 基本的に後手の勝ちだが、上手いことkを設定すれば先手勝ちにできるかも
        for j in (0..n).rev() {
            if cv.get(&an[j]).unwrap()%2 != 0 {
                println!("{}", an[j]-1);
                return;
            }
        }
        println!("{}", 0);
        return;
        
    } else {
        // 基本的に先手の勝ち。うまくkを設定すればよい
        // kは山の数より多ければいいので最大値は存在しない。
        println!("{}", -1);        
    }
}