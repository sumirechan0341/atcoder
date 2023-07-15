use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32,
        m: i32,
        q: usize,
        abcdq: [(i32, i32, i32, i32); q]
    };
    let cases = dfs(1, vec![], n, m);
    let mut max = 0;
    for case in cases {
        let mut total = 0;
        for (a, b, c, d) in &abcdq {
            if case[*b as usize -1] - case[*a as usize -1] == *c {
                total += d;
            }
        }
        if total > max {
            max = total;
        }
    }
    println!("{}", max);
}

fn dfs(most_right_digit: i32, now: Vec<i32>, n: i32, m: i32) -> Vec<Vec<i32>> {
    if now.len() == n as usize {
        return vec![now];
    }
    let mut res = vec![];
    for i in most_right_digit..=m {
        let mut next = now.clone();
        next.push(i);
        res.push(dfs(i, next, n, m));
    }
    return res.into_iter().flatten().collect::<Vec<_>>();
}