use proconio::input;

pub fn main() {
    input! {
        a: i32,
        p: i32
    };
    println!("{}", (3 * a + p) / 2);
}