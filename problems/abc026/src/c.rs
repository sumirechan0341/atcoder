use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        bn1: [usize; n-1]
    };
    let mut used = vec![false; n];
    let mut kyuryo = vec![0; n];
    for i in (0..n).rev() {
        if i != n-1 {
            used[bn1[i]-1] = true;
        }
        if used[i] {
            let mut max = std::i64::MIN;
            let mut min = std::i64::MAX;
            for j in i..n-1 {
                if bn1[j]-1 == i {
                    if kyuryo[j+1] > max {
                        max = kyuryo[j+1];
                    }
                    if kyuryo[j+1] < min {
                        min = kyuryo[j+1];
                    }
                }
            }
            kyuryo[i] = max+min+1;
        } else {
            kyuryo[i] = 1;
        }
    }
    println!("{}", kyuryo[0]);
}