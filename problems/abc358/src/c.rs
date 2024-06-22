use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        sn: [Chars; n]
    };
    let mut ans = n as u32;
    for i in 0..1 << n {
        let mut v = vec![false; m];
        for j in 0..n {
            if i >> j & 1 == 1 {
                for k in 0..m {
                    if sn[j][k] == 'o' {
                        v[k] = true;
                    }
                }
            }
        }
        if v.iter().all(|&x| x) {
            ans = ans.min((i as usize).count_ones());
        }
    }
    println!("{}", ans);
}
