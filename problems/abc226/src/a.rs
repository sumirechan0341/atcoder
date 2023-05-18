use proconio::input;

pub fn main() {
    input! {
        x: f32
    };
    println!("{}", x.round());
}