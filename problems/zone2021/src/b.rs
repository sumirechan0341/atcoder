use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        d: f32,
        h: f32,
        dhn: [(f32, f32); n]
    };
    let mut max = 0.0;
    for (di, hi) in dhn {
        let x = if h / d > hi / di {
            0.0
        } else {
            h - (h-hi)/(d-di)*d
        };
        if max < x {
            max = x;
        }
    }
    println!("{}", max);
}