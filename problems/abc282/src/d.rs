use std::collections::VecDeque;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        uvm: [(usize, usize); m]
    };
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        graph[uvm[i].0-1].push(uvm[i].1-1);
        graph[uvm[i].1-1].push(uvm[i].0-1);
    }
    let mut bipartite = vec![];
    let mut used: Vec<i64> = vec![-1; n];
    let mut ans: i64 = 0;
    for i in 0..n {
        if used[i] != -1 {
            continue;
        }
        let mut local = vec![];
        let mut queue = VecDeque::new();
        let mut ok = true;
        let mut count = vec![0; 2];
        let mut e_num = 0;
        queue.push_back((i, 0));
        used[i] = 0;
        local.push(i);
        count[0] += 1;
        while let Some((current, cost)) = queue.pop_front() {
            for &next in &graph[current] {
                // BFSで辺の数を数える
                // usedかどうか関係なくとりあえず辺の数を数えて最後に2で割る
                e_num += 1;
                if used[next] != -1 {
                    
                    if used[next]%2 == cost%2 {
                        ok = false;
                        continue;
                    } else {
                        continue;
                    }                    
                }
                count[((cost+1)%2) as usize] += 1;
                used[next] = cost+1;
                local.push(next);
                queue.push_back((next, cost+1));
            }
            
        }
        if ok {
            bipartite.push(local.len() as i64);
            ans += count[0]*count[1]-e_num/2;
        } else {
            println!("{}", 0);
            return;
        }

    }
    let sum = bipartite.iter().sum::<i64>();
    let sum2 = bipartite.iter().map(|x| x*x).sum::<i64>();
    println!("{}", ans + (sum*sum-sum2)/2 );
}

