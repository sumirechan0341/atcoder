use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        s: i32,
        t: i32,
        a: [i32; n]
    };
    let mut ans = 0;
    let mut weight = a[0];
    if s <= weight && weight <= t {
        ans += 1;
    } 
    for i in 1..n {
        weight += a[i];
        if s <= weight && weight <= t {
            ans += 1;
        } 
    }
    println!("{}", ans);
}