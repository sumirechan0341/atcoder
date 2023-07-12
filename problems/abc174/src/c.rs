use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32
    };
    if n==1 || n == 7 {
        println!("{}", 1);
        return;
    }
    let mut mod_total = 7;
    let mut mod_prev = 7;
    for i in 1..=n {
        let mod_now = (mod_prev * 10) % n;
        mod_prev = mod_now;
        mod_total += mod_now;
        mod_total %= n;
        if mod_total == 0 {
            println!("{}", i+1);
            return;
        }
    }
    println!("{}", -1);
    return;
}