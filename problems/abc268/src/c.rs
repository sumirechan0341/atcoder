use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        pn: [usize; n]
    };
    let mut max = 0;
    let mut chain = 0;
    let mut dx = 0;
    
    for i in 0..n {
        if pn[i] + 1 != pn[(i+1)%n] {
            chain = 0;
        } else {
            chain += 1;
            if chain > max {
                max = chain;
                dx = (n+i-pn[i])%n;
            }
        }
    }
    
}