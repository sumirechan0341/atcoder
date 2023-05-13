use proconio::input;

pub fn main() {
    input! {
        n: u32,
    }
    for i in 0..=n {
        println!("{}", n-i);
    }
}