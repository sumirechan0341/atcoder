use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        wxn: [(i64, i64); n]
    };
    let mut max = 0;
    for i in 0..24 {
        let mut local = 0;
        for j in 0..n {
            if (wxn[j].1+i)%24 >= 9 && (wxn[j].1+i)%24 <= 17 {
                local += wxn[j].0;
            }
        }
        if local > max {
            max = local;
        }
    }
    println!("{}", max);
}