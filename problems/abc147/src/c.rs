 use proconio::{input, marker::Chars};
 
pub fn main() {
    input! {
        n: usize
    };
    let mut graph = vec![vec![0; n]; n];
    for i in 0..n {
        input! {
            a: usize,
            xya: [(usize, i32); a]
        };
        for (x, y) in xya {
            if y == 1 {
                graph[i][x-1] = 1;
            } else {
                graph[i][x-1] = -1;
            }
        }
        graph[i][i] = 1;
    }
    let mut max = 0;
    for i in 0..2_i32.pow(n as u32) {
        let mut ii = i;
        let mut indices = vec![];
        for j in 0..n {
            if ii & 1 == 1 {
                indices.push(j);
            }
            ii = ii >> 1;
        }
        // 全パターン試すので正直者の数が厳密に一致しなければ、失敗扱いでよい
        // 正直者グラフが閉じていて、中に不親切な人がいなければ無矛盾
        let mut ok = true;
        for j in &indices {
            for k in 0..n {
                if graph[*j][k] == 1 && !indices.contains(&k) {
                    ok = false;
                    break;
                }
                if graph[*j][k] == -1 && indices.contains(&k) {
                    ok = false;
                    break;
                }
            }
            if !ok {
                break;
            }
        }
        if ok && max < i.count_ones() {
            max = i.count_ones();
        }
    }
    println!("{}", max);
}