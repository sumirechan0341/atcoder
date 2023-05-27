use proconio::input;

pub fn main() {
    input !{
        a: i32,
        b: i32,
        c: i32,
        d: i32
    };
    println!("{}", if (a - c).abs() <= d || ((b-a).abs() <= d && (c-b).abs() <= d) { "Yes" } else { "No" });
}