use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut dn: [i32; n]
    };
    dn.sort();
    println!("{}", dn[n/2]-dn[n/2-1]);
}