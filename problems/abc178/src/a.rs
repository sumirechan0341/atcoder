use proconio::input;

pub fn main() {
    input !{
        x: i32
    };
    println!("{}", if x == 0 { 1 } else { 0 });
}