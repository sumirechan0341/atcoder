use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        d: usize,
        sn: [Chars; n]
    };
    let mut max = 0;
    let mut streak = 0;
    for day in 0..d {
        let mut ok = true;
        for i in 0..n {
            if sn[i][day] == 'x' {
                ok = false;
                break;
            }
        }
        if ok {
            streak += 1;
            if streak > max {
                max = streak;
            }
        } else {
            streak = 0;
        }
    }
    println!("{}", max);
}