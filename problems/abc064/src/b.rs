use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut an: [i32; n]
    };
    an.sort();
    println!("{}", an.last().unwrap() - an[0]);
}