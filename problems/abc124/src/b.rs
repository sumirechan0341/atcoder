use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        hn: [i32; n]
    };
    let mut count = 0;
    for i in 0..n {
        let mut ok = true;
        for j in 0..i {
            if hn[i] < hn[j] {
                ok = false;
            }
        }
        if ok {
            count += 1;
        }
    }
    println!("{}", count);
}