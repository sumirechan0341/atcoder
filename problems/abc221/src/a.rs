use proconio::input;

pub fn main() {
    input! {
        a: u32,
        b: u32
    };
    println!("{}", 32_u64.pow(a - b));
}