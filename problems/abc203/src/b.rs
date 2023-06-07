use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32,
        k: i32
    };
    let mut ans = 0;
    for i in 1..n+1 {
        for j in 1..k+1 {
            ans += 100*i + j;
        }
    }
    println!("{}", ans);
}