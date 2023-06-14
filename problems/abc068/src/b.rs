use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32
    };
    let d = (0..7).map(|x| 2_i32.pow(x)).filter(|x| x <= &n).collect::<Vec<_>>();
    println!("{}", d.last().unwrap());
}