use proconio::{input, marker::Chars};
use itertools::Itertools;
pub fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
        cdm: [(usize, usize); m]
    };
    for c in (0..n).permutations(n) {
        let mut graph1 = vec![vec![]; m];
        let mut graph2 = vec![vec![]; m];
        for i in 0..m {
            graph1[c[abm[i].0-1]].push(c[abm[i].1-1]);
            graph1[c[abm[i].1-1]].push(c[abm[i].0-1]);
            graph2[c[cdm[i].0-1]].push(c[cdm[i].1-1]);
            graph2[c[cdm[i].1-1]].push(c[cdm[i].0-1]);
        }
        graph1.sort();
        graph2.sort();
        for i in 0..m {
            graph1[i].sort();
            graph2[i].sort();
        }
        
        if is_isomorphic(graph1, graph2, n) {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}
fn is_isomorphic(g1: Vec<Vec<usize>>, g2: Vec<Vec<usize>>, n: usize) -> bool {
    for i in 0..n {
        if g1[i] != g2[i] {
            return false;
        }
    }
    return true;
}