use proconio::{input, marker::Chars, input_interactive};

pub fn main() {
    input! {
        n: i64
    };
    let mut primes = vec![];
    let mut prime2 = vec![];
    for i in 2..=1000000 {
        if is_prime(i) {
            primes.push(i);
            prime2.push(i*i);
        }
    }
    let mut ans = 0;
    for i in 0.. {
        if primes[i] > 300 {
            break;
        }
        for j in i+1..primes.len() {
            let c = n / (primes[i]*primes[i]*primes[j]);
            if c >= 4 {
                match prime2[j+1..].binary_search(&c) {
                    Ok(x) => ans += x+1,
                    Err(x) => ans += x
                }
            }
            
        }
    }
    println!("{}", ans);
}

fn is_prime(n: i64) -> bool {
    if n < 6 {
        return n == 2 || n == 3 || n == 5;
    } else if n%6 != 1 && n%6 != 5 {
        return false;
    } else {
        for i in 2.. {
            if i*i > n {
                break;
            }
            if n%i == 0 {
                return false;
            }
        }
        return true;
    }
}