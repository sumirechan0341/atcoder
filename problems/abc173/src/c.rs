use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
        w: usize,
        k: i32,
        chw:[Chars; h]
    };
    let mut count = 0;
    for a in 0..2_i32.pow(h as u32) {
        for b in 0..2_i32.pow(w as u32) {
            let mut local_count = 0;
            let mut hd = a;
            for i in 0..h {
                if hd & 1 == 0 {
                    hd = hd >> 1;
                    continue;
                }
                let mut wd = b;
                for j in 0..w {
                    if wd & 1 == 0 {
                        wd = wd >> 1;
                        continue;
                    }
                    if chw[i][j] == '#' {
                        local_count += 1;
                    }
                    wd = wd >> 1;
                }
                hd = hd >> 1;
            }
            if local_count == k {
                count += 1;
            }   
        }
    }
    println!("{}", count);
}