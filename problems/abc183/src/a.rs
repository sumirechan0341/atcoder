use proconio::input;

pub fn main() {
    input! {
        x: i32
    };
    println!("{}", x.max(0));
}