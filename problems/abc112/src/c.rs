use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        xyhn: [(i32, i32, i32); n]
    };
    
    for cx in 0..=100 {
        for cy in 0..=100 {
            let mut ch = 0;
            for (x, y, h) in &xyhn {
                if *h == 0 {
                    continue;
                }
                ch = (cx-*x).abs() + (cy-*y).abs() + h
            }
            let mut ok = true;
            for i in 0..n {
                if (ch - (xyhn[i].0 - cx).abs() - (xyhn[i].1 - cy).abs()).max(0) != xyhn[i].2 {
                    ok = false;
                    break;
                }
            }
            if ok {
                println!("{} {} {}", cx, cy, ch);
                return;
            }
        }
    }
}