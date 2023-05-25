use proconio::input;

pub fn main() {
    input! {
        k: i32,
        x: i32
    };
    println!("{}", if k * 500 >= x { "Yes" } else { "No" });
}