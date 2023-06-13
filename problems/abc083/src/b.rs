use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32
    };
    println!("{}", (1..=n).filter(|x| a <= f(*x) && f(*x) <= b).sum::<i32>());
}

fn f(n: i32) -> i32 {
    let mut ans = 0;
    let mut a = n;
    while a != 0 {
        ans += a % 10;
        a /= 10;
    }
    return ans;
} 