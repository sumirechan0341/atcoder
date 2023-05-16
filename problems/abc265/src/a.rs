use proconio::input;

pub fn main() {
    input! {
        x: u32,
        y: u32,
        n: u32
    }
    if y / 3 < x {
        println!("{}", y * (n / 3) + x * (n % 3));
    } else {
        println!("{}", x * n);
    }
}