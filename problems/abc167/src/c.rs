use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        x: i32,
        camn: [(i32, [i32; m]); n]
    };
    let mut min = -1;
    for i in 0..2_i32.pow(n as u32) {
        let mut algo_pow = vec![0; m];
        let mut cost = 0;
        let mut ii = i;
        let mut ok = true;
        for j in 0..n {
            if ii & 1 == 1 {
                for k in 0..m {
                    algo_pow[k] += camn[j].1[k];
                }
                cost += camn[j].0;
            }
            ii = ii >> 1;
        }
        for j in 0..m {
            if algo_pow[j] < x {
                ok = false;
                break;
            }
        }
        if ok && (min > cost || min == -1) {
            min = cost;
        }
    }
    println!("{}", min);
}