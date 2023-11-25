use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        l: i32,
        an: [i32; n]
    };
    println!("{}", an.iter().filter(|&x| x >= &l).count());
}
