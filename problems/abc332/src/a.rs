use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        s: usize,
        k: usize,
        pqn: [(usize, usize); n],
    };
    let tot = pqn.iter().map(|(p, q)| p * q).sum::<usize>();
    println!("{}", if tot >= s { tot } else { tot + k });
}
