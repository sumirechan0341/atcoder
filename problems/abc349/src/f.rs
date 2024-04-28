use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: u64,
        mut m: u64,
        an: [u64; n]
    };
    let mut fs = vec![];
    let ps = sieve(100000000);
    for p in ps {
        let mut e = 0;
        if m % p == 0 {
            while m % p == 0 {
                m /= p;
                e += 1;
            }
            fs.push((p, e));
        }
    }
    let mut used = vec![0; fs.len()];
    let mut dp = vec![vec![0; 60]; fs.len() + 1];
    dp[1][0] = 1;
    for a in an {}
}

/// 素数のみを含んだ配列を生成する。nは10^6くらいの入力が限界
fn sieve(n: u64) -> Vec<u64> {
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
