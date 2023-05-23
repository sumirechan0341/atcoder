use proconio::input;

pub fn main() {
    input! {
        a1: [i32; 2],
        a2: [i32; 2]
    };
    println!("{}", a1[0] * a2[1] - a1[1] * a2[0]);
}