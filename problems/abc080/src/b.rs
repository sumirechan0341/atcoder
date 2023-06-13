use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32
    };
    println!("{}", if n % f(n) == 0 { "Yes" } else { "No" });
}

fn f(n: i32) -> i32 {
    let mut a = n;
    let mut ans = 0;
    while a != 0 {
        ans += a % 10;
        a /= 10;
    }
    return ans;
}