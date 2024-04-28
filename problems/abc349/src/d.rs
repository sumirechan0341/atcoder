use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        l: u64,
        r: u64,
    };
    println!("{}", divide_into_good_sequences(l, r).len());
    for (l, r) in divide_into_good_sequences(l, r) {
        println!("{} {}", l, r);
    }
}

fn divide_into_good_sequences(l: u64, r: u64) -> Vec<(u64, u64)> {
    let mut result = Vec::new();
    let mut now = l;
    if l == 0 {
        now = 1;
        while now << 1 <= r {
            now = now << 1;
        }
        result.push((0, now));
    }
    while now < r {
        let mut mrb = get_mrb(now);
        while now + mrb > r {
            mrb /= 2;
        }
        let next = now + mrb;
        result.push((now, next));
        now = next;
    }
    result
}

fn get_mrb(n: u64) -> u64 {
    let mut n = n;
    let mut result = 1;
    while n > 0 {
        if n & 1 == 1 {
            return result;
        }
        n /= 2;
        result *= 2;
    }
    return result;
}
