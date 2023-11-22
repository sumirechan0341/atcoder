use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut s: Chars
    };
    let goal = ['a', 't', 'c', 'o', 'd', 'e', 'r'];
    let mut ans = 0;
    for i in 0..s.len() {
        for j in (i+1..s.len()).rev() {
            if s[j] == goal[i] {
                s.swap(j, j-1);
                ans += 1;
            }
        }
        
    }
    println!("{}", ans);
}