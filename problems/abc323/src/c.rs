use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        mut am: [i32; m],
        sn: [Chars; n]
    };
    let mut score = vec![0; n];
    for i in 0..n {
        score[i] += i as i32+1;
    }
    for i in 0..n {
        for j in 0..m {
            score[i] += if sn[i][j] == 'o' { am[j] } else { 0 };
        }
    }
    let max = score.iter().max().unwrap();
    for i in 0..n {
        let mut remain = vec![];
        for j in 0..m {
            if sn[i][j] == 'x' {
                remain.push(am[j]);
            }
        }
        remain.sort();
        remain.reverse();
        let mut diff = max - score[i];
        let mut count = 0;
        for j in 0..remain.len() {
            if diff <= 0 {
                break;
            }
            diff -= remain[j];
            count += 1;
        }
        println!("{}", count);
    }
}