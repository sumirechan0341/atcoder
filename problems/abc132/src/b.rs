use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        pn: [i32; n]
    };
    let mut count = 0;
    for i in 1..n-1 {
        if pn[i-1] < pn[i] && pn[i] < pn[i+1] || pn[i-1] > pn[i] && pn[i] > pn[i+1] {
            count += 1;
        }
    }
    println!("{}", count);
}