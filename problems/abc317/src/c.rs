use proconio::{input, marker::Chars};
use itertools::Itertools;
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        abcm: [(usize, usize, i64); m]
    };
    let mut graph = vec![vec![-1; n+1]; n+1];
    for &(a, b, c) in &abcm {
        graph[a][b] = c;
        graph[b][a] = c;
    }
    let mut max = 0;
    for perm in (1..=n).permutations(n) {
        let mut local = 0;
        for i in 0..n-1 {
            if graph[perm[i]][perm[i+1]] == -1 {
                break;
            } else {
                local += graph[perm[i]][perm[i+1]]
            }
        }
        if max < local {
            max = local
        }
    }
    println!("{}", max)
}