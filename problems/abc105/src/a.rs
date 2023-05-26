use proconio::input;

pub fn main() {
    input! {
        n: i32,
        k: i32
    };
    println!("{}", if n % k == 0 { 0 } else { 1 });
}