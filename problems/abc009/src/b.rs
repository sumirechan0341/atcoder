use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut an: [i32; n]
    };
    an.sort();
    println!("{}", an.iter().filter(|x| x != &an.last().unwrap()).collect::<Vec<_>>().last().unwrap());
}