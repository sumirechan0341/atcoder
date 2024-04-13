use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        s: Chars,
        cn: [usize; n]
    };
    // 01
    let mut rui1 = vec![0; n + 1];
    // 10
    let mut rui2 = vec![0; n + 1];
    for i in 0..n {
        if i % 2 == 0 && s[i] != '0' {
            rui1[i + 1] = rui1[i] + cn[i];
            rui2[i + 1] = rui2[i];
        }
        if i % 2 == 0 && s[i] != '1' {
            rui2[i + 1] = rui2[i] + cn[i];
            rui1[i + 1] = rui1[i];
        }
        if i % 2 == 1 && s[i] != '1' {
            rui1[i + 1] = rui1[i] + cn[i];
            rui2[i + 1] = rui2[i];
        }
        if i % 2 == 1 && s[i] != '0' {
            rui2[i + 1] = rui2[i] + cn[i];
            rui1[i + 1] = rui1[i];
        }
    }
    let mut ans = usize::MAX;
    if n == 2 {
        if s == vec!['0', '1'] || s == vec!['1', '0'] {
            println!("{}", cn.iter().min().unwrap());
            return;
        } else {
            println!("{}", 0);
            return;
        }
    }
    for i in 1..n - 1 {
        if ans > rui2[i] + rui1[n] - rui1[i + 1] {
            ans = rui2[i] + rui1[n] - rui1[i + 1];
        }
        if ans > rui1[i] + rui2[n] - rui2[i + 1] {
            ans = rui1[i] + rui2[n] - rui2[i + 1];
        }
    }
    println!("{}", ans);
}
