use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        k: usize,
        g: usize,
        m: usize
    };
    let mut gg = 0;
    let mut mm = 0;
    for i in 0..k {
        if gg == g {
            gg = 0;
        } else if mm == 0 {
            mm = m;
        } else {
            let t = (g - gg).min(mm);
            gg += t;
            mm -= t;
        }
    }
    println!("{} {}", gg, mm);
}
