use proconio::{input, marker::Chars};
use itertools::Itertools;
pub fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uvwm: [(usize, usize, usize); m]
    };
    let mut graph = vec![vec![!0; n+1]; n+1];
    for (u, v, w) in uvwm {
        graph[u][v] = w;
        graph[v][u] = w;
    }
    let mut min = std::usize::MAX/2;
    for perm in (1..=n).permutations(n) {
        let mut costs = vec![];
        costs.push(0);
        let mut ok = true;
        for i in 0..perm.len()-1 {
            let mut local_cost = vec![];
            let mut local = false;
            for j in i+1..n {
                if graph[perm[i]][perm[j]] != !0 {
                    local = true;
                    for l in 0..costs.len() {
                        local_cost.push((costs[l]+graph[perm[i]][perm[j]])%k);
                    }
                }
                
            }
            if !local {
                ok = false;
                break;
            }
            costs = local_cost;
        }
        if ok && min > *costs.iter().min().unwrap() {
            min = *costs.iter().min().unwrap();
        }
    }
    println!("{}", min);
}