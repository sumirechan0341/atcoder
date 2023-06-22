use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: u64
    };
    let ans = &mut HashSet::<u64>::new();
    add_candidate(n, 0, ans);
    ans.insert(0);
    let mut ans_vec = ans.iter().collect::<Vec<&u64>>();
    ans_vec.sort();
    for a in ans_vec {
        println!("{}", a);
    }
}

fn add_candidate(n: u64, d: u32, ans: &mut HashSet<u64>) {
    if n < 2_u64.pow(d)-1 {
        return;
    }
    let candidate = n & (2_u64.pow(d+1)-1);
    if (n >> d) & 1 == 1 {
        // 1でも0でも
        // target_d以下のbitは保存
        ans.insert(candidate);
        ans.insert(candidate - 2_u64.pow(d));
        add_candidate(n, d+1, ans);
        add_candidate(n-2_u64.pow(d), d+1, ans);
    } else {
        // 0のみ
        add_candidate(n, d+1, ans)
    }
}