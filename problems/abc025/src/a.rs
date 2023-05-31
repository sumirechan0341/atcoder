use proconio::{input, marker::Chars};
pub fn main() {
    input! {
        s: Chars,
        n: usize
    };
    let mut dict = Vec::new();
    for i in 0..5 {
        let a = s[i].to_string();
        for j in 0..5 {
            dict.push(a.clone() + &s[j].to_string());
        }
    }
    println!("{}", dict[n-1]);
}