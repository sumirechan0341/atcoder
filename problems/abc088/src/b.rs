use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut an: [i32; n]
    };
    an.sort();
    an.reverse();
    let mut ans = 0;
    for i in 0..n {
        if i % 2 == 0 {
            ans += an[i];
        } else {
            ans -= an[i];
        }
    }
    println!("{}", ans);
}