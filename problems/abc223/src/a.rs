use proconio::input;

pub fn main() {
    input! {
        x: u32
    };
    println!("{}", if x % 100 == 0 && x != 0 { "Yes" } else { "No" });
}