use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        csfn: [(i64, i64, i64); n-1]
    };
    for i in 0..n-1 {
        let mut total = 0;
        for j in i..n-1 {
            if total <= csfn[j].1 {
                total = csfn[j].1 + csfn[j].0;
            } else {
                total += (csfn[j].2-total%csfn[j].2)%csfn[j].2 + csfn[j].0;
            }
        }
        println!("{}", total);
    }
    println!("{}", 0);
}