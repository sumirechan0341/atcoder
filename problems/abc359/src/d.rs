use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars
    };
    let mut dp = vec![vec![0; 1 << k]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        for mask in 0..(1 << k) {
            if dp[i][mask] == 0 {
                continue;
            }

            let next_char_options: HashSet<u8> = match s[i] {
                'A' => vec![0].into_iter().collect(),
                'B' => vec![1].into_iter().collect(),
                _ => vec![0, 1].into_iter().collect(),
            };

            for &next_char in &next_char_options {
                let next_mask = ((mask << 1) | (next_char as usize)) & ((1 << k) - 1);

                if i + 1 >= k {
                    let mut substring = Vec::with_capacity(k);
                    for j in 0..k {
                        substring.push(((next_mask >> j) & 1) as u8);
                    }

                    if is_palindrome(&substring) {
                        continue;
                    }
                }

                dp[i + 1][next_mask] = (dp[i + 1][next_mask] + dp[i][mask]) % 998244353;
            }
        }
    }

    println!("{}", dp[n].iter().fold(0, |acc, &x| (acc + x) % 998244353));
}

fn is_palindrome(substring: &Vec<u8>) -> bool {
    let len = substring.len();
    for i in 0..len / 2 {
        if substring[i] != substring[len - 1 - i] {
            return false;
        }
    }
    true
}
