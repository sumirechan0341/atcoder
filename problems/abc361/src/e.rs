use proconio::input;
use proconio::marker::Usize1;

fn find(parent: &mut Vec<usize>, x: usize) -> usize {
    if parent[x] != x {
        parent[x] = find(parent, parent[x]);
    }
    parent[x]
}

fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
    let root_x = find(parent, x);
    let root_y = find(parent, y);
    if root_x != root_y {
        if rank[root_x] > rank[root_y] {
            parent[root_y] = root_x;
        } else if rank[root_x] < rank[root_y] {
            parent[root_x] = root_y;
        } else {
            parent[root_y] = root_x;
            rank[root_x] += 1;
        }
    }
}

fn bfs(start: usize, graph: &Vec<Vec<(usize, usize)>>) -> (usize, usize) {
    let n = graph.len();
    let mut dist = vec![-1; n];
    let mut queue = std::collections::VecDeque::new();
    dist[start] = 0;
    queue.push_back(start);

    let mut farthest_node = start;
    while let Some(node) = queue.pop_front() {
        for &(neighbor, cost) in &graph[node] {
            if dist[neighbor] == -1 {
                dist[neighbor] = dist[node] + cost as i64;
                queue.push_back(neighbor);
                if dist[neighbor] > dist[farthest_node] {
                    farthest_node = neighbor;
                }
            }
        }
    }
    (farthest_node, dist[farthest_node] as usize)
}

pub fn main() {
    input! {
        n: usize,
        roads: [(Usize1, Usize1, usize); n-1],
    }

    // 辺リストの作成
    let mut edges = roads.clone();

    // Kruskalのアルゴリズムのために辺を重みでソート
    edges.sort_by_key(|&(_, _, cost)| cost);

    // Union-Find データ構造の初期化
    let mut parent = (0..n).collect::<Vec<_>>();
    let mut rank = vec![0; n];

    let mut total_cost = 0;
    let mut mst_edges = vec![];

    // Kruskalのアルゴリズムで最小全域木を構築
    for (a, b, cost) in edges {
        if find(&mut parent, a) != find(&mut parent, b) {
            union(&mut parent, &mut rank, a, b);
            total_cost += cost;
            mst_edges.push((a, b, cost));
        }
    }

    // MSTを使ってグラフを構築
    let mut graph = vec![vec![]; n];
    for (a, b, cost) in mst_edges {
        graph[a].push((b, cost));
        graph[b].push((a, cost));
    }

    // 任意の点 (0) から最も遠い点を見つける
    let (farthest_node, _) = bfs(0, &graph);

    // その点から最も遠い点を見つける (木の直径の一端)
    let (_, diameter) = bfs(farthest_node, &graph);

    // 木の総コストの2倍から直径を引く
    let total_traversal_cost = total_cost * 2 - diameter;

    println!("{}", total_traversal_cost);
}
