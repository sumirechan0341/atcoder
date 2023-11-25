use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        sn: [Chars; n]
    };
    let mut yoko_cnt = vec![0; n];
    let mut tate_cnt = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if sn[i][j] == 'o' {
                yoko_cnt[i] += 1;
                tate_cnt[j] += 1;
            }
        }
    }
    let mut ans = 0i64;
    for i in 0..n {
        for j in 0..n {
            if sn[i][j] != 'o' {
                continue;
            }
            ans += (yoko_cnt[i] - 1) * (tate_cnt[j] - 1);
        }
    }
    println!("{}", ans);
}
