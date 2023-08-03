use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        txyn: [(i32, i32, i32); n]
    };
    let mut prev_p = (0, 0);
    let mut prev_t = 0;
    for (t, x, y) in txyn {
        let t = t - prev_t;
        let x = (x-prev_p.0).abs();
        let y = (y-prev_p.1).abs();
        if t < x + y || t%2 != (x+y)%2 {
            println!("{}", "No");
            return;
        }
        prev_p = (x, y);
        prev_t = t;
    }
    println!("{}", "Yes");
}