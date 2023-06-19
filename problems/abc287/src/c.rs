use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        uvm: [(usize, usize); m]
    };
    // 次数1の点が2個と次数2の点が残りみたいな方法にしたほうがよかった
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for uv in uvm {
        graph[uv.0-1].push(uv.1-1);
        graph[uv.1-1].push(uv.0-1);
    }
    if n - m != 1 {
        println!("{}", "No");
        return;
    }
    let ok = &mut 0;
    let used = &mut vec![false; n];
    dfs(0, &graph, used, ok);
    if *ok > 1 {
        println!("{}", "No");
        return;
    }
    for i in 0..n {
        if !used[i] {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}
fn dfs(v: usize, graph: &Vec<Vec<usize>>, used: &mut Vec<bool>, ok: &mut i32) {
    if used[v] {
        return;
    }
    used[v] = true;
    let mut deg = 0;
    for next in &graph[v] {
        if used[*next] {
            continue;
        }
        deg += 1;
        dfs(*next, graph, used, ok);
    }
    if deg > 2 {
        *ok = 2;
    }
    if deg == 2 {
        *ok += 1;
    }
}