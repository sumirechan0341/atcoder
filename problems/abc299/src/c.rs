use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        s: Chars
    };
    let mut max: i32 = 0;
    let mut lev: i32 = 0;
    for i in 0..n {
        if s[i] == 'o' {
            lev += 1;
        } else {
            if lev > 0 {
                if max < lev {
                    max = lev;
                }
                lev = 0;
            }
        }
        if i == n-1 {
            if lev != n as i32 && lev > max {
                max = lev
            }
        }
    }
    
    println!("{}", if max == 0 { -1 } else { max });
}