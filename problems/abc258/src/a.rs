use proconio::input;

pub fn main() {
    input! {
        k: u32
    };
    println!("{:02}:{:02}", 21 + k / 60, k % 60);
}