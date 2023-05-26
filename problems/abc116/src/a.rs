use proconio::input;

pub fn main() {
    input! {
        ab: i32,
        bc: i32,
        _ca: i32
    };
    println!("{}", ab * bc / 2);
}