use proconio::input;

pub fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32
    };
    println!("{}", (b / a).min(c));
}