use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [i64; n]
    };
    let mut cnt = vec![0; 200001];
    for i in 0..n {
        cnt[an[i] as usize] += 1;
    }
    let mut ans = 0;
    // 1だけのグループ
    ans += (cnt[1] - 2).max(0) * (cnt[1] - 1).max(0) * cnt[1];
    for i in 0..n {
        for p in 2..=448 {
            if p * p > an[i] {
                break;
            }
            if an[i] % p == 0 {
                if an[i] == p {
                    ans += cnt[1] * (cnt[an[i] as usize] - 1) * 2;
                } else {
                    ans += cnt[p as usize] * cnt[(an[i] / p) as usize] * 2;
                }
            }
        }
    }

    println!("{}", ans);
}
