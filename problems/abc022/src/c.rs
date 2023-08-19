use proconio::{input, marker::Chars};
use itertools::Itertools;
pub fn main() {
    input! {
        n: usize,
        m: usize,
        uvlm: [(usize, usize, i64); m]
    };
    // 解説AC
    let mut d = vec![vec![-1; n+1]; n+1];
    for &(u, v, l) in &uvlm {
        if u == 1 {
            continue;
        }
        d[u][v] = l;
        d[v][u] = l;
    }
    for k in 1..=n {
        for i in 1..=n {
            for j in 1..=n {
                if d[i][k] == -1 || d[k][j] == -1 {
                    continue;
                }
                if d[i][j] == -1 {
                    d[i][j] = d[i][k]+d[k][j];
                } else {
                    d[i][j] = d[i][j].min(d[i][k]+d[k][j]);
                }
            }
        }
    }
    let mut min = -1;
    let adj1 = uvlm.iter().filter(|x| x.0 == 1).collect::<Vec<_>>();
    for comb in (0..adj1.len()).combinations(2) {
        let x = adj1[comb[0]];
        let y = adj1[comb[1]];
        if d[x.1][y.1] > 0 && (min == -1 || x.2 + d[x.1][y.1] + y.2 < min) {
            min = x.2 + d[x.1][y.1] + y.2;
        }
    }
    println!("{}", min);
}