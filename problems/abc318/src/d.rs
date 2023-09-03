// use std::collections::HashSet;

// use proconio::{input, marker::Chars};
// type VS = Vec<String>;

// pub fn main() {
//     input!{
//         n: usize,
//         d: [i64; n*(n-1)/2]
//     };
//     let mut graph = vec![vec![0; n]; n];
//     let mut count = 0;
//     for i in 0..n-1 {
//         for j in i+1..=n-1 {
//             graph[i][j] = d[count];
//             graph[j][i] = d[count]; 
//             count += 1;
//         }
//     }
//     let mut max = 0;
//     let mut used = vec![false; n];
//     if n%2 == 0 {
//         max = max.max(dfs(&graph, n, &used));
//     } else {
//         for i in 0..n {
//             used[i] = true;
//             max = max.max(dfs(&graph, n, &used));
//             used[i] = false;
//         }
//     }
    
//     println!("{}", max);
// }

// fn dfs(graph: &Vec<Vec<i64>>, n: usize, used: &Vec<bool>) -> i64 {
//     if used.iter().all(|&x| x) {
//         return 0;
//     }
//     let mut u = !0;
//     for i in 0..n {
//         if !used[i] {
//             u = i;
//             break;
//         }
//     }
//     let mut new_used = used.clone();
//     new_used[u] = true;
//     let mut max = 0;
//     for v in u+1..n {
//         if new_used[v] {
//             continue;
//         }
//         new_used[v] = true;
//         let candidate = graph[u][v] + dfs(graph, n, &new_used);
//         if max < candidate {
//             max = candidate;
//         }
//         new_used[v] = false;
//     }
//     return max;
// }

use proconio::{input, marker::Chars};
use std::cmp::*;
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        d: [i64; n*(n-1)/2]
    };
    let mut graph = vec![vec![0; n]; n];
    let mut count = 0;
    for i in 0..n-1 {
        for j in i+1..=n-1 {
            graph[i][j] = d[count];
            graph[j][i] = d[count]; 
            count += 1;
        }
    }
    let mut dp = vec![0; 1<<n];
    for i in 0..1<<n {
        let mut u = !0;
        for j in 0..n {
            if 1 & (i>>j) != 1 {
                u = j;
                break;
            }
        }
        if u == !0 {
            break;
        }
        for j in u+1..n {
            if 1 & (i>>j) != 1 {
                dp[i|1<<u|1<<j] = dp[i|1<<u|1<<j].max(dp[i] + graph[u][j]);
            }
        }
    }
    println!("{}", dp[(1<<n)-1]);
}