use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut w: i128,
        mut abn: [(i128, i128); n]
    };
    abn.sort();
    abn.reverse();
    let mut deliciousness = 0;
    for ab in abn {
        deliciousness += ab.0 * ab.1.min(w);
        w -= ab.1;
        if w <= 0 {
            break;
        }
    }
    println!("{}", deliciousness);
}