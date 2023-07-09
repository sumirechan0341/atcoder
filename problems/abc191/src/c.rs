use std::collections::HashMap;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        h: usize,
        w: usize,
        shw: [Chars; h]
    };
    // 多角形の問題は次数に注目することで解決することもあるかも
    let mut vertices = vec![vec![false; w+3]; h+3];
    let mut edge = HashMap::<(usize, usize), Vec<(usize, usize)>>::new();
    for i in 0..h {
        for j in 0..w {
            if shw[i][j] == '#' {
                vertices[i+1][j+1] = true;
                vertices[i+2][j+1] = true;
                vertices[i+1][j+2] = true;
                vertices[i+2][j+2] = true;

                edge.entry((i+1, j+1)).and_modify(|x| x.push((i+1, j+2))).or_insert(vec![(i+1, j+2)]);
                edge.entry((i+1, j+1)).and_modify(|x| x.push((i+2, j+1))).or_insert(vec![(i+2, j+1)]);
                edge.entry((i+2, j+1)).and_modify(|x| x.push((i+2 ,j+2))).or_insert(vec![(i+2 ,j+2)]);
                edge.entry((i+2, j+2)).and_modify(|x| x.push((i+1, j+2))).or_insert(vec![(i+1, j+2)]);

                edge.entry((i+1, j+2)).and_modify(|x| x.push((i+1, j+1))).or_insert(vec![(i+1, j+1)]);
                edge.entry((i+2, j+1)).and_modify(|x| x.push((i+1, j+1))).or_insert(vec![(i+1, j+1)]);
                edge.entry((i+2 ,j+2)).and_modify(|x| x.push((i+2, j+1))).or_insert(vec![(i+2, j+1)]);
                edge.entry((i+1, j+2)).and_modify(|x| x.push((i+2, j+2))).or_insert(vec![(i+2, j+2)]);
            }
        }
    }
    let mut count = 0;
    for i in 1..h+2 {
        for j in 1..w+2 {
            if vertices[i][j] {
                let mut degree = 0;
                let empty = vec![];
                let edges = edge.get(&(i, j)).unwrap_or(&empty);
                if vertices[i+1][j] && edges.contains(&(i+1, j)) {
                    degree += 1;
                }
                if vertices[i-1][j] && edges.contains(&(i-1, j)) {
                    degree += 1;
                }
                if vertices[i][j-1] && edges.contains(&(i, j-1)) {
                    degree += 1;
                }
                if vertices[i][j+1] && edges.contains(&(i, j+1)) {
                    degree += 1;
                }
                // println!("({}, {}) deg: {}", i, j, degree);
                if degree == 2 {
                    count += 1;
                }
                if degree == 4 {
                    if shw[i-2][j-2] == '#' && shw[i-2][j-1] == '#' && shw[i-1][j-2] == '#' && shw[i-1][j-1] == '#' {
                        // 完全に内部に含まれる点はカウントしない
                    } else {
                        count += 1;
                    }
                }
            }
        }
    }
    println!("{}", count);
}