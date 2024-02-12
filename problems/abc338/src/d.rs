use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        xm: [usize; m]
    };
    let mut imos: Vec<i64> = vec![0; n + 1];
    for i in 0..m - 1 {
        let diff = if xm[i + 1] > xm[i] {
            xm[i + 1] - xm[i]
        } else {
            n - xm[i] + xm[i + 1]
        } as i64;
        let diff_inv = n as i64 - diff;
        if diff == diff_inv {
            continue;
        }
        // 逆方向が最適解
        if diff > diff_inv {
            // 1-----xm[i] ----- xm[i+1]---n
            // 逆方向の道中がカットされたときの余計なコストを加算
            if xm[i] < xm[i + 1] {
                imos[0] += diff - diff_inv;
                imos[xm[i]] -= diff - diff_inv;
                imos[xm[i + 1]] += diff - diff_inv;
                imos[n] -= diff - diff_inv;
            } else {
                // xm[i+1] ----- xm[i]
                imos[xm[i + 1]] += diff - diff_inv;
                imos[xm[i]] -= diff - diff_inv;
            }
        } else {
            // 順方向が最適解
            // xm[i] ----- xm[i+1]
            // 順方向の道中がカットされたときの余計なコストを加算
            if xm[i] < xm[i + 1] {
                imos[xm[i]] += diff_inv - diff;
                imos[xm[i + 1]] -= diff_inv - diff;
            } else {
                // 1-----xm[i+1] ----- xm[i]----n
                imos[0] += diff_inv - diff;
                imos[xm[i + 1]] -= diff_inv - diff;
                imos[xm[i]] += diff_inv - diff;
                imos[n] -= diff_inv - diff;
            }
        }
    }
    let mut min = 0;
    for i in 0..m - 1 {
        let diff = (xm[i + 1] as i64 - xm[i] as i64).abs();
        min += (n as i64 - diff).min(diff) as i64;
    }
    for i in 0..n {
        imos[i + 1] += imos[i];
    }
    println!("{}", min + imos[0..n].iter().min().unwrap());
}
