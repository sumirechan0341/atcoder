use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        b23: [[i32; 3]; 2],
        c32: [[i32; 2]; 3]
    };
    // 解説AC
    let score = dfs(vec![vec![-1; 3]; 3], &b23, &c32, 0);
    let total = b23.iter().flatten().sum::<i32>() + c32.iter().flatten().sum::<i32>();
    println!("{}", (total+score)/2);
    println!("{}", (total-score)/2);
}
fn dfs(used: Vec<Vec<i32>>, b23: &Vec<Vec<i32>>, c32: &Vec<Vec<i32>>, turn: i32) -> i32 {
    if used.iter().flatten().all(|&x| x!=-1) {
        let mut score = 0;
        for i in 0..2 {
            for j in 0..3 {
                if used[i][j] == used[i+1][j] {
                    score += b23[i][j];
                } else {
                    score -= b23[i][j];
                }
            }
        }
        for i in 0..3 {
            for j in 0..2 {
                if used[i][j] == used[i][j+1] {
                    score += c32[i][j];
                } else {
                    score -= c32[i][j];
                }
            }
        }
        return score;
    }
    let mut scores = vec![];
    for i in 0..3 {
        for j in 0..3 {
            if used[i][j] != -1 {
                continue;
            }
            let mut next = used.clone();
            next[i][j] = turn;
            scores.push(dfs(next, b23, c32, turn^1));
        }
    }
    return if turn == 0 { *scores.iter().max().unwrap() } else { *scores.iter().min().unwrap() }
}