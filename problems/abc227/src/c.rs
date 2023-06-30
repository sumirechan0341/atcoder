use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: u64
    };
    let mut count = 0;
    for a in 1..=n {
        if a*a*a > n {
            break;
        }
        for b in a..=n {
            if a*b*b > n {
                break;
            }
            count += n / (a*b) - b + 1;
        }
    }
    println!("{}", count);
}