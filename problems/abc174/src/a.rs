use proconio::input;

pub fn main() {
    input! {
        x: i32
    };
    println!("{}", if x >= 30 { "Yes" } else { "No" });
}