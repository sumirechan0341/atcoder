use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        xyn: [(i64, i64); n]
    };
    for i in 0..n {
        let mut max = 0;
        let mut idx = 0;
        for j in 0..n {
            if i == j {
                continue;
            }
            if max
                < (xyn[j].1 - xyn[i].1) * (xyn[j].1 - xyn[i].1)
                    + (xyn[j].0 - xyn[i].0) * (xyn[j].0 - xyn[i].0)
            {
                max = (xyn[j].1 - xyn[i].1) * (xyn[j].1 - xyn[i].1)
                    + (xyn[j].0 - xyn[i].0) * (xyn[j].0 - xyn[i].0);
                idx = j + 1;
            }
        }
        println!("{}", idx);
    }
}
