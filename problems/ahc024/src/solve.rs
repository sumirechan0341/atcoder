use std::collections::{HashSet, HashMap, VecDeque};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        cnn: [[i32; n]; n]
    };
    let mut ans = cnn.clone();
    // 3x3のマス目で真ん中を抜いても連結情報は変わらない
    // ただし外周に接している色のみ可能
    let mut zero_conneted = HashSet::<i32>::new();
    for i in 0..n {
        zero_conneted.insert(cnn[0][i]);
        zero_conneted.insert(cnn[n-1][i]);
        zero_conneted.insert(cnn[i][0]);
        zero_conneted.insert(cnn[i][n-1]);
    }
    let mut color_map = vec![HashSet::<(usize, usize)>::new(); m+1];
    let mut candidates = vec![vec![false; n]; n];
    for i in 0..n {
        for j in 0..n {
            if let Some(local) = window(3, 3, (i, j), &cnn) {
                if composed_by_one_color(local) && zero_conneted.contains(&cnn[i][j]) {
                    candidates[i+1][j+1] = true;
                    color_map[cnn[i][j] as usize].insert((i+1, j+1));
                }
            }
        }
    }
    // TODO 連結でない0の領域を連結にすることができるか考える
    // 白くする領域のうち、連結でないものは白くしないようにする
    for i in 0..n {
        for j in 0..n {
            if candidates[i][j] {
                if candidates[i+1][j] || candidates[i-1][j] || candidates[i][j+1] || candidates[i][j-1] {

                } else {
                    candidates[i][j] = false;
                    color_map[cnn[i][j] as usize].remove(&(i, j));
                }
            }
        }
    }
    let mut ng_color = vec![false; m+1];
    for color_idx in 1..=m {
        ng_color[color_idx] = !color_map[color_idx].iter().any(|x| x.0 == 1 || x.0 == n-2 || x.1 == 1 || x.1 == n-2)
    }
    // 端の点について、その隣の点が白くならないと白くならないみたいなパターンがあるので2回やる
    for k in 0..2 {
        let mut resolved = vec![false; m+1];
        for i in 0..n {
            for j in 0..n {
                if !(i == 0 || i == n-1 || j == 0 || j == n-1) {
                    continue;
                }
                // 左隣は白
                if i == 0 {
                    // 上は白
                    if j == 0 {
                        if (candidates[i+1][j]) && (candidates[i][j+1]) {
                            candidates[i][j] = true;
                        }
                    }
                    // 下は白
                    else if j == n-1 {
                        if (candidates[i+1][j]) && (candidates[i][j-1]) {
                            candidates[i][j] = true;
                        }
                    }
                    else {
                        if (candidates[i+1][j]) && (cnn[i][j-1] == cnn[i][j+1]) {
                            candidates[i][j] = true;
                        }
                    } 
                }
                // 右隣りは白
                else if i == n-1 {
                    // 上は白
                    if j == 0 {
                        if (candidates[i-1][j]) && (candidates[i][j+1]) {
                            candidates[i][j] = true;
                        }
                    }
                    // 下は白
                    else if j == n-1 {
                        if (candidates[i-1][j]) && (candidates[i][j-1]) {
                            candidates[i][j] = true;
                        }
                    }
                    else if (candidates[i-1][j]) && (cnn[i][j-1] == cnn[i][j+1]) {
                        candidates[i][j] = true;
                    }
                } else {
                    // 上は白
                    if j == 0 {
                        if (cnn[i-1][j] == cnn[i+1][j]) && (candidates[i][j+1]) {
                            candidates[i][j] = true;
                        }
                    }
                    // 下は白
                    else if j == n-1 {
                        if (cnn[i-1][j] == cnn[i+1][j]) && (candidates[i][j-1]) {
                            candidates[i][j] = true;
                        }
                    }
                    else if (cnn[i-1][j] == cnn[i+1][j]) && (cnn[i][j-1] == cnn[i][j+1]) {
                        candidates[i][j] = true;
                    }
                }
                
            }
        }
    }

    let mut colored_map = vec![HashSet::<(usize, usize)>::new(); m+1];
    for i in 0..n {
        for j in 0..n {
            if !candidates[i][j] {
                colored_map[cnn[i][j] as usize].insert((i, j));
            }
        }
    }

    for color_idx in 1..=m {
        if ng_color[color_idx] {
            continue;
        }
        // 白マスが連結か
        let whites = &color_map[color_idx];
        let mut used = vec![];
        let mut queue = VecDeque::<(usize, usize)>::new();
        if whites.len() == 0 {
            continue;
        }
        queue.push_back(*(whites.iter().nth(0).unwrap()));
        while let Some(current) = queue.pop_front() {
            if used.contains(&current) {
                continue;
            }
            used.push(current);
            if whites.contains(&(current.0+1, current.1)) {
                queue.push_back((current.0+1, current.1));
            }
            if whites.contains(&(current.0-1, current.1)) {
                queue.push_back((current.0-1, current.1));
            }
            if whites.contains(&(current.0, current.1+1)) {
                queue.push_back((current.0, current.1+1));
            }
            if whites.contains(&(current.0, current.1-1)) {
                queue.push_back((current.0, current.1-1));
            }
        }
        if whites.len() != used.len() {
            ng_color[color_idx] = true;
            continue;
        }

        // 色マスが連結か
        let colored = &colored_map[color_idx];
        let mut used = vec![];
        let mut queue = VecDeque::<(usize, usize)>::new();
        if colored.len() == 0 {
            continue;
        }
        queue.push_back(*(colored.iter().nth(0).unwrap()));
        while let Some(current) = queue.pop_front() {
            if used.contains(&current) {
                continue;
            }
            used.push(current);
            if colored.contains(&(current.0+1, current.1)) && !whites.contains(&(current.0+1, current.1)) {
                queue.push_back((current.0+1, current.1));
            }
            if current.0 > 0 && colored.contains(&(current.0-1, current.1)) && !whites.contains(&(current.0-1, current.1)) {
                queue.push_back((current.0-1, current.1));
            }
            if colored.contains(&(current.0, current.1+1)) && !whites.contains(&(current.0, current.1+1)) {
                queue.push_back((current.0, current.1+1));
            }
            if current.1 > 0 && colored.contains(&(current.0, current.1-1)) && !whites.contains(&(current.0, current.1-1)) {
                queue.push_back((current.0, current.1-1));
            }
        }
        if colored.len() != used.len() {
            ng_color[color_idx] = true;
        }
    }
    

    // println!("{:?}", ng_color);
    for i in 0..n {
        for j in 0..n {
            if candidates[i][j] && !ng_color[cnn[i][j] as usize] {
                print!("0 ");
            } else {
                print!("{} ", cnn[i][j]);
            }
        }
        println!("{}", "");
    }
    
}

// data must be n x n vector
// originは左上
// 0-indexed
fn window(h: usize, w: usize, origin: (usize, usize), data: &Vec<Vec<i32>>) -> Option<Vec<Vec<i32>>> {
    if origin.0 + h > data.len() || origin.1 + w > data.len() {
        return None;
    }
    let mut res = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            res[i][j] = data[i+origin.0][j+origin.1];
        }
    }
    return Some(res);
}

// いろいろな形に対応
// 長方形でなくても使える
fn composed_by_one_color(v: Vec<Vec<i32>>) -> bool {
    let mut set = HashSet::<i32>::new();
    for i in 0..v.len() {
        for &c in &v[i] {
            set.insert(c);
        }
    }
    return set.len() == 1;
}