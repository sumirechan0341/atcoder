use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        t: usize,
        p: usize,
        mut ln: [usize; n]
    };
    ln.sort();
    ln.reverse();
    println!("{}", if ln[p - 1] >= t { 0 } else { t - ln[p - 1] });
}
