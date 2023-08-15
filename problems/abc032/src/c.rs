use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        k: i64,
        sn: [i64; n]
    };
    if sn.contains(&0) {
        println!("{}", n);
        return;
    }
    if k == 0 {
        println!("{}", 0);
        return;
    }
    let mut max = 0;
    let mut r = 0;
    let mut local = 1;
    let mut div_flag = false;
    for l in 0..n {
        r = r.max(l);
        while local*sn[r] <= k {
            div_flag = true;
            local *= sn[r];
            if max < r-l+1 {
                max = r-l+1;
            }
            if r == n-1 {
                break;
            }
            r += 1;
            
        }
        if div_flag {
            local /= sn[l];
        }
    }
    println!("{}", max);
}