use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        xyn: [(usize, usize); n]
    };
    let mut t = 0;
    let mut a = 0;
    for i in 0..n {
        t += xyn[i].0;
        a += xyn[i].1;
    }
    println!(
        "{}",
        if t > a {
            "Takahashi"
        } else if t < a {
            "Aoki"
        } else {
            "Draw"
        }
    );
}
