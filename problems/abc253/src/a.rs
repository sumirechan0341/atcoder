use proconio::input;

pub fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32
    };
    println!("{}", if a <= b && b <= c || c <= b && b <= a  { "Yes" } else { "No" });
}