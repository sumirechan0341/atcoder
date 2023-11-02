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

fn is_prime_table(n: i64) -> Vec<bool> {
    let mut prime = vec![true; (n+1) as usize];
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
        for j in (i+1..=n).step_by(i as usize) {
            prime[j as usize] = false;
        }
    }
    return prime;
}
