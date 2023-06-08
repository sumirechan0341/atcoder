use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut an: [u128; n]
    };
    an.sort();
    an.reverse();
    if an[n-1] == 0 {
        println!("{}", 0);
        return;
    }
    let mut acc = 1;
    for a in an {
        acc *= a;
        if acc > 10_u128.pow(18) {
            println!("{}", -1);
            return;
        }
    }
    println!("{}", acc);
}