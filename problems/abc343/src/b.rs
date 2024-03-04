use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        ann: [[i64; n]; n]
    };
    for i in 0..n {
        let mut local = vec![];
        for j in 0..n {
            if ann[i][j] == 1 {
                local.push((j + 1).to_string());
            }
        }
        println!("{}", local.join(" "));
    }
}
