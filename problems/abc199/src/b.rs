use proconio::{input, marker::Chars};
use itertools::zip;
pub fn main() {
    input! {
        n: usize,
        an: [i32; n],
        bn: [i32; n]
    };
    let abn = zip(an, bn);
    let mut count = 0;
    for i in 1..1001 {
        let mut ok = true;
        for ab in abn.clone() {
            if !(ab.0 <= i && i <= ab.1) {
                ok = false;
                break;
            }
        }
        if ok {
            count += 1;
        }
    }
    println!("{}", count);
}