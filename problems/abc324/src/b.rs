use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut n: i64
    };
    while n != 1 {
        let mut ok = false;
        if n % 2 == 0 {
            ok = true;
            n /= 2;
        }
        if n % 3 == 0 {
            ok = true;
            n /= 3;
        }
        if !ok {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}