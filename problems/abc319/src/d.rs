use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: i64,
        ln: [i64; n]
    };
    let mut start = 0;
    let mut end = std::i64::MAX;
    while end-start > 1 {
        let med = (start + end)/2;
        let mut local = 0;
        let mut lines = 1;
        let mut ok = true;
        for i in 0..n {
            if ln[i] > med {
                ok = false;
            }
            if local + ln[i] > med {
                lines += 1;
                local = 0;
            }
            local += ln[i] + 1;
        }
        if !ok {
            start = med;
            continue;
        }
        if lines > m {
            start = med;
        } else {
            end = med;
        }
    }
    println!("{}", end);
}