use proconio::input;

pub fn main() {
    input !{
        a: i32,
        b: i32
    };
    println!("{}", if b >= a { a } else { a - 1 });
}