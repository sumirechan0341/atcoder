use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: String
    };
    let t = "oxx".repeat(10_i32.pow(5) as usize);
    println!("{}", if t.contains(&s) { "Yes" } else { "No" });
}