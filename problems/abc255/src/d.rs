use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        q: usize,
        mut an: [i64; n],
        xq: [i64; q]
    };
    an.sort();
    let med = an[n/2];
    let mut min_cost = 0;
    for i in 0..n {
        min_cost += (an[i] - med).abs();
    }
    let mut sn = vec![0; n+1];
    for i in 0..n {
        sn[i+1] = sn[i] + (an[i]-med).abs();
    }
    let an2 = an.iter().map(|x| x*2).collect::<Vec<_>>();
    for x in xq {
        if let Err(y) = an2.binary_search(&(2*x-1)) {
            println!("{}", (med-x).abs()*(((n-y).max(y)-(n-y).min(y)) as i64) - (sn[(n/2).max(y)]-sn[(n/2).min(y)])*2 + min_cost);
        }
    }
}