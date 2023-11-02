use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        k: i64
    };
    let primes = sieve(1000000);
    let mut kk = k;
    let mut factros = vec![];
    for p in primes {
        let mut acc = 0;
        while kk%p == 0 {
            kk /= p;
            acc += 1;
        }
        if acc > 0 {
            factros.push((p, acc));
        }
        if kk == 1 {
            break;
        }
    }
    if kk != 1 {
        factros.push((kk, 1))
    }
    // if factros.is_empty() {
    //     println!("{}", k);
    //     return;
    // }
    let mut max = 2;
    for (p, e) in factros {
        let mut unsat = 1;
        let mut sat = p.pow(e as u32);
        while sat-unsat > 1 {
            let mid = (sat+unsat)/2;
            if count_factor_of_factorial(mid, p) >= e {
                sat = mid;
            } else {
                unsat = mid;
            }
        }
        if sat > max {
            max = sat;
        }
    }
    println!("{}", max);
}

fn count_factor_of_factorial(n: i64, p: i64) -> i64 {
    let mut ans = 0;
    let mut pp = p;
    while pp <= n {
        ans += n/pp;
        pp *= p;
    }
    return ans;
}

fn sieve(n: i64) -> Vec<i64> {
    let mut prime = vec![true; (n+1) as usize];
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