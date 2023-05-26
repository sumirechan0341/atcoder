use proconio::input;

pub fn main() {
    input! {
        a: i32,
        b: i32
    };
    println!("{}", if a == b { 2 * a } else { (2*a-1).max(2*b-1) });
}