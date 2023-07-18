use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        x: i32
    };
    for n in x.. {
        if is_prime(n) {
            println!("{}", n);
            return;
        }
    }
}

fn is_prime(n: i32) -> bool {
    for i in 2.. {
        if i*i > n {
            break;
        }
        if n % i == 0 {
            return false;
        }
    }
    return true;
}