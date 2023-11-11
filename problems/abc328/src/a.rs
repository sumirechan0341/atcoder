use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        x: i32,
        sn: [i32; n]
    };
    let mut ans = 0;
    for i in 0..n {
        if sn[i] <= x {
            ans += sn[i];
        }
    }
    println!("{}", ans);
}