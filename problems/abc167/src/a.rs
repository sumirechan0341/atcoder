use proconio::input;

pub fn main() {
    input! {
        s: String,
        t: String
    };
    println!("{}", if s[0..s.len()] == t[0..t.len()-1] { "Yes" } else { "No" });
}