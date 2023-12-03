use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mm: usize,
        dd: usize,
        mut y: usize,
        mut m: usize,
        mut d: usize
    };
    d += 1;
    if dd + 1 == d {
        d = 1;
        m += 1;
    }
    if mm + 1 == m {
        m = 1;
        y += 1;
    }
    println!("{} {} {}", y, m, d);
}
