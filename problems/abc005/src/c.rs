use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        t: usize,
        n: usize,
        an: [usize; n],
        m: usize,
        bm: [usize; m]
    };
    let mut ta = vec![0; 101];
    for i in 0..n {
        ta[an[i]] += 1;
    }
    for i in 0..m {
        let mut ng = true;
        for j in 0..=bm[i] {
            if ta[j] > 0 {
                if j+t < bm[i] {
                    ta[j] = 0;
                    continue;
                }
                ta[j] -= 1;
                ng = false;
                break;
            }
        }
        if ng {
            println!("{}", "no");
            return;
        }
    }
    println!("{}", "yes");
}