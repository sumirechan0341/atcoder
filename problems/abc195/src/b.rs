use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i32,
        b: i32,
        w: i32
    };
    let large = a.max(b);
    let small = a.min(b);
    let mut min = 0;
    let mut max = 0;
    let mut ok = false;
    for i in 1.. {
        if small * i <= w * 1000 && w * 1000 <= large * i {
            if !ok {
                min = i;
                ok = true;
            }
        } else {
            if ok {
                println!("{} {}", min, i-1);
                return;
            }
        }
        if large * i < w * 1000 && w * 1000 < small * (i + 1) {
            println!("{}", "UNSATISFIABLE");
            return;
        } 
    }
}