use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        p3n: [[i32; 3]; n] 
    };
    let mut score = p3n.iter().map(|x| x[0] + x[1] + x[2]).collect::<Vec<_>>();
    score.sort();
    score.reverse();
    let reference = score[k-1];
    for i in 0..n {
        if p3n[i][0] + p3n[i][1] + p3n[i][2] + 300 < reference {
            println!("{}", "No");
        } else {
            println!("{}", "Yes");
        }
    }
}