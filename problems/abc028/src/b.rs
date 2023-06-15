use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    let mut ans = vec![0; 6];
    for c in s {
        ans[(c as u8 - 65) as usize] += 1;
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
} 