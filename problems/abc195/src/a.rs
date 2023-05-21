use proconio::input;

pub fn main() {
    input !{
        m: i32,
        h: i32
    };
    println!("{}", if h % m == 0 { "Yes" } else { "No" } );
}