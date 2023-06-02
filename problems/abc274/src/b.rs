use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
        w: usize,
        chw: [Chars; h]
    };
    let mut counts = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            if chw[i][j] == '#' {
                counts[j] += 1;
            }
        }
    }
    println!("{}", counts.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}