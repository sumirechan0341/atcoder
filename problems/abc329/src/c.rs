use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        s: Chars
    };
    let mut count = vec![0; 26];
    count[s[0] as usize - 'a' as usize] = 1;
    let mut suc = 1;
    for i in 1..n {
        if s[i] == s[i-1] {
            suc += 1;
        } else {
            suc = 1;
        }
        count[s[i] as usize - 'a' as usize] = count[s[i] as usize - 'a' as usize].max(suc);
    }
    let mut ans = 0;
    for i in 0..26 {
        ans += count[i];
    }
    println!("{}", ans);
}