use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        k: usize,
        an: [i64; n]
    };
    let p = 1000000007;
    let mut s = vec![0; n];
    let mut fst = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            } else if i < j {
                if an[i] > an[j] {
                    fst[i] += 1;
                    s[i] += 1;
                } 
            } else {
                if an[i] > an[j] {
                    s[i] += 1;
                }
            }
            
        }
    }
    let mut total = 0;
    for i in 0..n {
        total += (((((s[i]*(k-1))%p)*k)%p)*500000004)%p + (k*fst[i]%p);
        total %= p;
    }
    println!("{}", total);
}