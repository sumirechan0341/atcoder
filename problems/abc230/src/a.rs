use proconio::input;

pub fn main() {
    input! {
        n: u32
    };
    println!("AGC{:03}", n + n / 42);
}