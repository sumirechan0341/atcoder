use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        s: Chars
    };
    let mut count = 0;
    let mut ans = 0i64;
    for i in 0..n-1 {
        if s[i] == '>' {
            count += 1;
        } else {
            ans += count*(count+1)/2;
            count = 0;
        }
    }
    ans += count*(count+1)/2;
    println!("{}", ans);
}