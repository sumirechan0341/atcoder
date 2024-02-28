use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        pn: [usize; n],
        q: usize,
        abq: [(usize, usize); q]
    };
    for (a, b) in abq {
        for i in 0..n {
            if pn[i] == a {
                println!("{}", a);
                break;
            } else if pn[i] == b {
                println!("{}", b);
                break;
            }
        }
    }
}
