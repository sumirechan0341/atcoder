use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
        k: usize,
        cdk: [(usize, usize); k]
    };
    let mut max = 0;
    for i in 0..2_i32.pow(k as u32) {
        let mut dish = vec![false; n+1];
        let mut ii = i;
        let mut count = 0;
        for j in 0..k {
            if ii & 1 == 1 {
                dish[cdk[j].1] = true;
            } else {
                dish[cdk[j].0] = true;
            }
            ii = ii >> 1;
        }
        for j in 0..m {
            if dish[abm[j].0] && dish[abm[j].1] {
                count += 1;
            }
        }
        if count > max {
            max = count;
        }
    }
    println!("{}", max);
}