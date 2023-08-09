use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [i64; n]        
    };
    let mut sn = vec![0; n+1];
    for i in 0..n {
        sn[i+1] = sn[i] + an[i];
    }
    // 奇数項+パターン
    let mut ruiseki = 0;
    let mut total1 = 0;
    for i in 0..n {
        if i%2 == 0 {
            if sn[i+1] + ruiseki <= 0 {
                total1 += 1-(sn[i+1] + ruiseki);
                ruiseki += 1-(sn[i+1] + ruiseki);
            }
        } else {
            if sn[i+1] + ruiseki >= 0 {
                total1 += sn[i+1] + ruiseki + 1;
                ruiseki -= sn[i+1] + ruiseki + 1;
            }
        }
    }
    // 奇数項-パターン
    ruiseki = 0;
    let mut total2 = 0;
    for i in 0..n {
        if i%2 == 0 {
            if sn[i+1] + ruiseki >= 0 {
                total2 += sn[i+1] + ruiseki + 1;
                ruiseki -= sn[i+1] + ruiseki + 1;
            }
        } else {
            if sn[i+1] + ruiseki <= 0 {
                total2 += 1-(sn[i+1] + ruiseki);
                ruiseki += 1-(sn[i+1] + ruiseki);
            }
        }
    }
    println!("{}", total1.min(total2));
}