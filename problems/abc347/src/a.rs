use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        an: [usize; n]
    };
    for a in an {
        if a % k == 0 {
            println!("{}", a / k);
        }
    }
}
