use proconio::input;

pub fn main() {
    input! {
        a: i32,
        b: i32
    };
    println!("{}", if a >= 13 { b } else if a >= 6 { b / 2 } else { 0 });
}