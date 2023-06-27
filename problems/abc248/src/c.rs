use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize
    };
    let p: i64 = 998244353;
    
    let mut dp = vec![vec![0; k+1]; n+1];
    for i in 1..=m {
        dp[1][i] = 1;
    }
    for i in 2..=n {
        for j in i..=k {
            println!("dp[{}][{}]", i, j);
            let low = if j < m {
                1
            } else {
                j-m
            };
            for l in low..m.min(j) {
                println!("add dp[{}][{}]", i-1, l);
                dp[i][j] += dp[i-1][l];
                dp[i][j] %= p;
            }
        }
    }
    let mut ans = 0;
    for i in 1..=k {
        ans += dp[n][i];
        ans %= p;
    }
    println!("{}", ans);
}

// pub fn pow_rem(base: u64, exp: u64, p: u64) -> u64 {
//     let mut ans = 1;
//     let mut k = exp;
//     let mut acc = base;
//     while k > 0 {
//         if k & 1 == 1 {
//             ans = (ans * acc) % p;
//         }
//         acc *= acc;
//         acc %= p;
//         k = k >> 1;
//     }
//     return ans;
// }

// pub fn comb_rem(n: u64, k: u64, p: u64) -> u64 {
//     let mut ans = 1;
//     for i in 0..k {
//         let inv = pow_rem(k-i, p-2, p);
//         ans *= n-i;
//         ans %= p;
//         ans *= inv;
//         ans %= p;
//     }
//     return ans;
// }