use proconio::input;

pub fn main() {
    input !{
        a: i32,
        b: i32,
        x: i32
    };
    println!("{}", if x > a + b || x < a { "NO" } else { "YES" } );
}