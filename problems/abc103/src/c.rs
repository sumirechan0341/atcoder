use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [i32; n]
    };
    println!("{}", an.iter().sum::<i32>()-n as i32);
}