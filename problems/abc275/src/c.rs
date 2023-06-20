use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::ops;
type VS = Vec<String>;

pub fn main() {
    input!{
        s9: [Chars; 9] 
    };
    // ベクトルで処理
    let mut vec_vertex = vec![];
    let mut ans = 0;
    for i in 0..9 {
        for j in 0..9 {
            if s9[i][j] == '#' {
                vec_vertex.push((i, j));
            }
        }
    }
    for c in (0..vec_vertex.len()).combinations(4) {
        // v1 dot v2 = 0
        // v2 dot v3 = 0
        // |v1| == |v2|
        // |v1| == |v3|
        // で十分
        let v1 = (vec_vertex[c[0]].0 as i32 - vec_vertex[c[1]].0 as i32, vec_vertex[c[0]].1 as i32 - vec_vertex[c[1]].1 as i32);
        let v2 = (vec_vertex[c[0]].0 as i32 - vec_vertex[c[2]].0 as i32, vec_vertex[c[0]].1 as i32 - vec_vertex[c[2]].1 as i32);
        let v3 = (vec_vertex[c[2]].0 as i32 - vec_vertex[c[3]].0 as i32, vec_vertex[c[2]].1 as i32 - vec_vertex[c[3]].1 as i32);

        // v1 dot v2
        if v1.0*v2.0 + v1.1*v2.1 != 0 {
            continue;
        }
        // v2 dot v3
        if v2.0*v3.0 + v2.1*v3.1 != 0 {
            continue;
        }
        // |v1| == |v2|
        if (v1.0.abs() + v1.1.abs()) != (v2.0.abs() + v2.1.abs()) {
            continue;
        }
        // |v1| == |v3|
        if (v1.0.abs() + v1.1.abs()) != (v3.0.abs() + v3.1.abs()) {
            continue;
        }
        ans += 1;
    }
    println!("{}", ans);
}