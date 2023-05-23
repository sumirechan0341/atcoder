use proconio::input;

pub fn main() {
    input! {
        x: i32
    };
    println!("{}", 100 - x % 100);
}