use proconio::input;

pub fn main() {
    input! {
        n: u32
    };
    println!("{}", if n == 2 || n == 3 || n == 4  { "No" } else { "Yes" });
}