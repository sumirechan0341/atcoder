use petgraph::{
    algo::{toposort, DfsSpace},
    graph::DiGraph,
    Directed, Graph,
};
use std::collections::VecDeque;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        an: [usize; n],
        uvm: [(usize, usize); m]
    };
    let g: petgraph::prelude::Graph<_, _, petgraph::prelude::Directed, usize> =
        DiGraph::from_edges(&uvm);
    let mut space = DfsSpace::new::<&Graph<_, _, Directed, usize>>(&g);
    let result = toposort::<&Graph<usize, (usize, usize), Directed, usize>>(&g, Some(&mut space));
    // let mut graph = vec![vec![]; n];
    // for i in 0..m {
    //     graph[uvm[i].0 - 1].push(uvm[i].1 - 1);
    //     graph[uvm[i].1 - 1].push(uvm[i].0 - 1);
    // }
    // let mut dp = vec![0; n];
    // let mut queue = VecDeque::new();
    // let mut used = vec![false; n];
    // queue.push_back((0, an[0]));
    // dp[0] = 1;
    // used[0] = true;
    // while let Some((u, max)) = queue.pop_front() {
    //     for &v in graph[u].iter() {
    //         if an[v] >= max {
    //             if an[v] != max {
    //                 dp[v] = (dp[u] + 1).max(dp[v]);
    //             } else {
    //                 dp[v] = dp[u].max(dp[v]);
    //             }
    //             if used[v] {
    //                 continue;
    //             }
    //             used[v] = true;
    //             queue.push_back((v, an[v]));
    //         }
    //     }
    // }
    // println!("{:?}", dp[n - 1]);
}
