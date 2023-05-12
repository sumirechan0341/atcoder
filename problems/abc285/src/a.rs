use proconio::input;

pub fn main() {
    input! {
        a: u8,
        b: u8,
    }
    if b >> 1 == a {
        println!("Yes");
    } else {
        println!("No");
    }
}