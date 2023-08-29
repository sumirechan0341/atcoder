use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32,
        m: i32
    };
    for x in 0..=m/4 {
        let remain = m - x*4;
        let mut start = 0;
        let mut end = remain/3;

        while end-start > 1 {
            let y = (start+end)/2;
            let z = (remain - y*3)/2;
            if x+y+z > n {
                start = y;
            } else {
                end = y;
            }
        }
        let y = end;
        let z = (remain-3*y)/2;
        if x+y+z == n && (remain-3*y) % 2 == 0 {
            println!("{} {} {}", z, y, x);
            return
        }

        let y = start;
        let z = (remain-3*y)/2;
        if x+y+z == n && (remain-3*y) % 2 == 0 {
            println!("{} {} {}", z, y, x);
            return
        }
    }
    println!("{} {} {}", -1, -1, -1);
}