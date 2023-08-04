use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut an: [i64; n],
        mut bn: [i64; n],
        mut cn: [i64; n],
    };
    let mut s1: Vec<i64> = vec![0; n+1];
    let mut s2: Vec<i64> = vec![0; n+1];
    an.sort();
    bn.sort();
    cn.sort();
    an.reverse();
    bn.reverse();
    cn.reverse();
    let mut cur = 0;
    for i in 0..n {
        s1[i+1] = s1[i];
        if cur == n {
            continue;
        }
        while an[i] < bn[cur] {
            s1[i+1] += 1;
            cur += 1;
            if cur == n {
                break;
            }
        }
    }
    cur = 0;
    for i in 0..n {
        s2[i+1] = s2[i];
        if cur == n {
            continue;
        }
        while bn[i] < cn[cur] {
            s2[i+1] += 1;
            cur += 1;
            if cur == n {
                break;
            }
        }
    }
    let mut ans = vec![0; n+1];
    for i in 0..n {
        ans[i+1] = ans[i] + s2[i+1];
    }
    let mut total = 0;
    for index in s1 {
        total += ans[index as usize];
    }
    println!("{}", total)
}