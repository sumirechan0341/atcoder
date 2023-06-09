use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32,
        k: i32,
        hn: [i32; n]
    };
    println!("{}", hn.into_iter().filter(|x| x >= &k).collect::<Vec<i32>>().len());
}