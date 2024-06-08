use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize
    };
    let l0 = vec![vec!['#']];
    let l1 = vec![
        vec!['#', '#', '#'],
        vec!['#', '.', '#'],
        vec!['#', '#', '#'],
    ];
    let mut l2 = vec![vec!['.'; 9]; 9];
    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                continue;
            }
            draw(&mut l2, &l1, i * 3, j * 3);
        }
    }
    let mut l3 = vec![vec!['.'; 27]; 27];
    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                continue;
            }
            draw(&mut l3, &l2, i * 9, j * 9);
        }
    }
    let mut l4 = vec![vec!['.'; 81]; 81];
    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                continue;
            }
            draw(&mut l4, &l3, i * 27, j * 27);
        }
    }
    let mut l5 = vec![vec!['.'; 243]; 243];
    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                continue;
            }
            draw(&mut l5, &l4, i * 81, j * 81);
        }
    }
    let mut l6 = vec![vec!['.'; 729]; 729];
    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                continue;
            }
            draw(&mut l6, &l5, i * 243, j * 243);
        }
    }
    let ans = vec![l0, l1, l2, l3, l4, l5, l6];
    println!(
        "{}",
        ans[n]
            .iter()
            .map(|v| v.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn draw(origin: &mut Vec<Vec<char>>, src: &Vec<Vec<char>>, i: usize, j: usize) {
    for k in 0..src.len() {
        for m in 0..src.len() {
            origin[i + k][j + m] = src[k][m];
        }
    }
}
