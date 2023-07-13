use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        k: i64,
        an: [i64; n],
        bm: [i64; m]
    };
    let mut sn = vec![0; n+1];
    let mut sm = vec![0; m+1];
    for i in 1..=n {
        sn[i] = sn[i-1] + an[i-1];
    }
    for i in 1..=m {
        sm[i] = sm[i-1] + bm[i-1];
    }
    let mut max = 0;
    for i in 0..=n {
        if sn[i] > k {
            break;
        }
        let remain = k - sn[i];
        match sm.binary_search(&remain) {
            Ok(x) => {
                if x + i + 1 > max {
                    max = x + i;
                }
            },
            Err(x) => {
                if x + i > max {
                    max = x + i - 1;
                }
            }
        }
    }
    println!("{}", max);
}