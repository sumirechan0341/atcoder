use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut n: usize
    };
    let mut ans = 0;
    while n != 0 {
        if n & 1 == 1 {
            break;
        } else {
            ans += 1;
        }
        n = n >> 1;
    }
    println!("{}", ans);
}
