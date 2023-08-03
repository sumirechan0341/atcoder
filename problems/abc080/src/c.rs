use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        f10n: [[i32; 10]; n],
        p11n: [[i32; 11]; n]
    };
    let mut max = std::i32::MIN;
    for i in 1..1024 {
        let mut ii = i;
        let mut total = 0;
        let mut c = vec![0; n];
        for j in 0..10 {
            if ii & 1 == 1 {
                for k in 0..n {
                    if f10n[k][j] == 1 {
                        c[k] += 1; 
                    }
                }
            }
            ii = ii >> 1;
        }
        for j in 0..n {
            total += p11n[j][c[j]];
        }

        if total > max {
            max = total;
        }
    }
    println!("{}", max);
}