use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut p: i32
    };
    let coins = (1..11).map(|x| fact(x)).collect::<Vec<_>>();
    let mut coin_num = 0;
    for i in (0..10).rev() {
        coin_num += p / coins[i]; 
        p %= coins[i];
    }
    println!("{}", coin_num);
}

fn fact(n: i32) -> i32 {
    if n == 0 {
        return 1;
    } else {
        return n * fact(n-1);
    }
}