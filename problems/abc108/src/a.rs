use proconio::input;

pub fn main() {
    input! {
        k: i32
    };
    println!("{}", (k + 1) / 2 * (k / 2));
}