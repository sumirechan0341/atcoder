use proconio::input;

pub fn main() {
    input! {
        s: String,
        t: String,
        a: i32,
        b: i32,
        u: String
    };
    println!("{} {}",  a - (s == u) as i32, b - (t == u) as i32 );
}