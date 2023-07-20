use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        hn: [i32; n]
    };
    let mut max = 0;
    let mut local = 0;
    for i in 0..n-1 {
        if hn[i] >= hn[i+1] {
            local += 1;
        } else {
            if local > max {
                max = local;
            }
            local = 0;
        }
    }
    if local > max {
        max = local;
    }
    println!("{}", max);
}