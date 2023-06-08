use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        mut pn: [u32; n]
    };
    pn.sort();
    println!("{}", pn.iter().take(k).sum::<u32>());
}