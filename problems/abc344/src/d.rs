use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        t: Chars,
        n: usize,
    };
    let mut ss = vec![];
    for _ in 0..n {
        input! {
            a: usize,
            s: [Chars; a]
        }
        ss.push(s);
    }
    let mut dp = vec![usize::MAX / 2; t.len() + 1];
    dp[0] = 0;
    for i in 0..n {
        let mut local_dp = dp.clone();
        for s in ss[i].clone() {
            for j in 0..t.len() {
                // dp[0] ~ dp[t.len()-1] まで更新する
                let mut ok = true;
                if j + s.len() > t.len() {
                    continue;
                }
                for k in 0..s.len() {
                    if t[j + k] != s[k] {
                        ok = false;
                        break;
                    }
                }
                if ok && dp[j + s.len()] > dp[j] + 1 {
                    local_dp[j + s.len()] = dp[j] + 1;
                }
            }
        }
        dp = local_dp;
    }
    if dp[t.len()] >= usize::MAX / 2 {
        println!("{}", -1);
        return;
    } else {
        println!("{}", dp[t.len()]);
    }
}
