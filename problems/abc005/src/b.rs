use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut tn: [i32; n]
    };
    tn.sort();
    println!("{}", tn[0]);
}