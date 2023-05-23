use proconio::input;

pub fn main() {
    input !{
        d: i32,
        t: i32,
        s: i32
    };
    println!("{}", if t * s >= d { "Yes" } else { "No" });
}