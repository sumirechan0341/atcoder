use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        r: i32,
        g: i32,
        b: i32
    };
    println!("{}", if ( 10 * g + b) % 4 == 0 { "YES" } else { "NO" } );
}