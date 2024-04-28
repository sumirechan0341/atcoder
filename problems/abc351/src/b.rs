use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        ann: [Chars; n],
        bnn: [Chars; n]
    };
    for i in 0..n {
        for j in 0..n {
            if ann[i][j] != bnn[i][j] {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}
