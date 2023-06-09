use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n],
        bn: [i32; n],
        cn1: [i32; n-1]
    };
    let mut prev = n;
    let mut s = 0;
    for a in an {
        s += bn[a-1];
        if prev + 1 == a-1 {
            s += cn1[prev];
        } 
        prev = a-1;
    }
    println!("{}", s);
}