use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
        w: usize,
        x: i32,
        y: i32,
        sh: [Chars; h]
    };
    let mut count = 0;
    let direction = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    if sh[(x-1) as usize][(y-1) as usize] == '.' {
        count += 1;
    }
    for d in direction {
        for k in 1..h.max(w) {
            let newx = x-1+k as i32*d.0;
            let newy = y-1+k as i32*d.1;
            
            if newx < 0 || newx > (h-1) as i32|| newy < 0 || newy > (w-1) as i32 {
                break;
            }
            if sh[newx as usize][newy as usize] == '.' {
                count += 1;
            } else {
                break;
            }
        }
        
    }
    println!("{}", count);
}