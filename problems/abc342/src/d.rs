use std::collections::HashMap;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [i64; n]
    };
    let mut pairs = HashMap::new();
    let mut factrial = vec![];
    for i in 0..n {
        factrial.push(vtostr(prime_factorization(an[i])));
    }
    let mut zero = 0;
    for i in 0..n {
        // 0
        if an[i] == 0 {
            zero += 1;
            continue;
        }
        // 完全数
        if an[i] == 1 || factrial[i] == "" {
            pairs
                .entry("完全数")
                .and_modify(|x| *x += 1)
                .or_insert(1_i64);
            continue;
        }
        // その他
        pairs
            .entry(&factrial[i])
            .and_modify(|x| *x += 1)
            .or_insert(1_i64);
    }
    let mut ans = 0;
    for (k, v) in pairs {
        ans += v * (v - 1) / 2
    }
    let mut c = 0;
    for i in 0..zero {
        ans += n as i64 - 1 - c;
        c += 1;
    }
    println!("{}", ans);
}
fn vtostr(v: Vec<i64>) -> String {
    let mut res = "".to_string();
    for i in v {
        res += &i.to_string();
        res += ",";
    }
    return res;
}

fn prime_factorization(mut n: i64) -> Vec<i64> {
    let mut res = vec![];
    let mut i = 2;
    while i * i <= n {
        let mut c = 0;
        while n % i == 0 {
            c += 1;
            n /= i;
        }
        if c % 2 == 1 {
            res.push(i);
        }
        i += 1;
    }
    if n != 1 {
        res.push(n);
    }
    return res;
}

/// 素数のみを含んだ配列を生成する。nは10^6くらいの入力が限界
fn sieve(n: i64) -> Vec<i64> {
    let mut prime = vec![true; (n + 1) as usize];
    let mut res = vec![];
    if n >= 0 {
        prime[0] = false;
    }
    if n >= 1 {
        prime[1] = false;
    }
    for i in 2..=n {
        if !prime[i as usize] {
            continue;
        }
        res.push(i);
        for j in (i..=n).step_by(i as usize) {
            prime[j as usize] = false;
        }
    }
    return res;
}
