use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        a2n: [[i32; n]; 2]
    };
    let mut max = 0;
    for i in 0..n {
        let mut local = 0;
        for j in 0..=i {
            local += a2n[0][j];
        }
        for j in i..n {
            local += a2n[1][j];
        }
        if max < local {
            max = local;
        }
    }
    println!("{}", max);
}