use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64
    };
    let primes = sieve(200);
    let mut win_t = false;
    for i in a..=b {
        let mut local_win_t = true;
        for j in c..=d {
            if primes.contains(&(i + j)) {
                local_win_t = false;
            }
        }
        if local_win_t {
            win_t = true;
        }
    }
    println!("{}", if win_t { "Takahashi" } else { "Aoki" });
}

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
