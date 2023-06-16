use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut s: Chars,
        n: usize,
        lrn: [(usize, usize); n]
    };
    for (l, r) in lrn {
        let ss = &mut s[l-1..r];
        (*ss).reverse();
    }
    println!("{}", s.iter().collect::<String>());
}