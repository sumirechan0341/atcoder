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
    let mut ans = 0i64;
    // 1だけのグループ
    ans += cnt[1] * cnt[1] * cnt[1];

    for i in 0..n {
        if an[i] == 1 {
            continue;
        }
        for p in 1..=448 {
            if p * p > an[i] {
                break;
            }
            if an[i] % p == 0 {
                if p == 1 {
                    // 自分自身をカウントから除く
                    ans += cnt[1] * (cnt[an[i] as usize]) * 2;
                } else if an[i] / p == p {
                    // 同じものを2個取っているので1個減らす
                    ans += cnt[p as usize] * (cnt[p as usize]);
                } else {
                    ans += cnt[p as usize] * cnt[(an[i] / p) as usize] * 2;
                }
            }
        }
    }
    println!("{}", ans);
}
