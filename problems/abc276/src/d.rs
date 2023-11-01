use proconio::{input, marker::Chars};
use num::{integer, Integer};
pub fn main() {
    input! {
        n: usize,
        mut an: [i32; n]
    };
    let mut gcd = an[0];
    for i in 0..n {
        gcd = gcd.gcd(&an[i]);
    }
    let mut ans = 0;
    for i in 0..n {
        while an[i]%3 == 0 && an[i] != gcd {
            if (an[i]/3).gcd(&gcd) != gcd {
                break;
            }
            an[i]/=3;
            ans += 1;
        }
        while an[i]%2 == 0 && an[i] != gcd {
            an[i]/=2;
            ans += 1;
        }
        if an[i] != gcd {
            println!("{}", -1);
            return;
        }
    }
    println!("{}", ans);
}