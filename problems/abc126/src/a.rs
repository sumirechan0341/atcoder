use proconio::input;

pub fn main() {
    input! {
        n: usize,
        k: usize,
        s: String
    };
    println!("{}", s[0..k-1].to_string() + &s.chars().nth(k-1).unwrap().to_lowercase().to_string() + &s[k..n]);
}