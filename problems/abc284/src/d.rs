use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        t: usize,
        testt: [i64; t],
    };
    let primes = sieve(3000000);
    // pかqは10^6までの検索で見つかる
    for test in testt {
        let mut ok = false;
        for &i in &primes {
            if test%i == 0 {
                let p2 = test/i;
                let mut start = 2;
                let mut end = p2+1;
                while end-start > 1 {
                    let mid = (start+end)/2;
                    if mid.saturating_mul(mid) > p2 {
                        end = mid;
                    } else {
                        start = mid; 
                    }
                }
                if p2 % (start*start) == 0 {
                    println!("{} {}", start, i);
                    ok = true;
                }                
            }
            if !ok && test%(i*i) == 0 {
                println!("{} {}", i, test/(i*i));
                ok = true;
            }
            if ok {
                break;
            }
        }
        
    }
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
