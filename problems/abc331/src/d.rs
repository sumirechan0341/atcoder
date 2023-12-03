use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        q: usize,
        pnn: [Chars; n],
        abcdq: [(usize, usize, usize, usize); q]
    };
}
