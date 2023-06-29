use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        q: usize,
        mut an: [i64; n],
        xq: [i64; q]
    };
    for i in 0..n {
        an[i] *= 10;
    }
    an.sort();
    for x in xq {
        if let Err(y) = an.binary_search(&(x*10-1)) {
            println!("{}", n-y);
        }
    }
}