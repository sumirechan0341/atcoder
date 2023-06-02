use proconio::{input, marker::Chars};
use itertools::Itertools;
pub fn main() {
    input! {
        n: usize,
        m: usize,
        sn: [Chars; n]
    };
    let mut count = 0;
    for c in (0..n).combinations(2) {
        let mut ok = true;
        for k in 0..m {
            if sn[c[0]][k] == 'x' && sn[c[1]][k] == 'x' {
                ok = false;
            }
        }
        if ok {
            count += 1;
        }
    }
    println!("{}", count);
}