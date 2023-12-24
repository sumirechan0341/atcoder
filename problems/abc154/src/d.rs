use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        pn: [i64; n]
    };
    let mut max = 0.0;
    let mut ssn = vec![0.0; n + 1];
    for i in 0..n {
        ssn[i + 1] = ssn[i] + f(pn[i]);
    }
    for i in 0..n - k + 1 {
        if max < (ssn[i + k] - ssn[i]) {
            max = ssn[i + k] - ssn[i];
        }
    }
    println!("{}", max);
}
fn f(k: i64) -> f64 {
    return (k * (k + 1) / 2) as f64 / k as f64;
}
