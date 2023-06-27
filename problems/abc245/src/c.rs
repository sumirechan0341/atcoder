use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        k: i32,
        an: [i32; n],
        bn: [i32; n]
    };
    let mut reachable = vec![vec![false; 2]; n];

    reachable[0][0] = true;
    reachable[0][1] = true;
    for i in 0..n-1 {
        if reachable[i][0] && (an[i] - an[i+1]).abs() <= k {
            reachable[i+1][0] = true;
        }
        if reachable[i][1] && (bn[i] - an[i+1]).abs() <= k {
            reachable[i+1][0] = true;
        }
        if reachable[i][0] && (an[i] - bn[i+1]).abs() <= k {
            reachable[i+1][1] = true;
        }
        if reachable[i][1] && (bn[i] - bn[i+1]).abs() <= k {
            reachable[i+1][1] = true;
        }
    }
    println!("{}", if reachable[n-1][0] || reachable[n-1][1] { "Yes" } else { "No" });
}