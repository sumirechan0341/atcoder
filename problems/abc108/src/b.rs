use std::mem::swap;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32
    };
    let mut dx = x2 - x1;
    let mut dy = y2 - y1;
    let current = 
    if dy > 0 && dx >= 0 {
        0
    } else if dx < 0 && dy >= 0 {
        1
    } else if dy < 0 && dx <= 0 {
        2
    } else {
        3
    };
    if current % 2 == 1 {
        let temp = dx;
        dx = dy;
        dy = temp;
    }
    dx = dx.abs();
    dy = dy.abs();
    let vs = [(dx, dy), (-dy, dx), (-dx, -dy), (dy, -dx)];

    let x3 = x2 + vs[(current+1)%4].0;
    let y3 = y2 + vs[(current+1)%4].1;
    let x4 = x3 + vs[(current+2)%4].0;
    let y4 = y3 + vs[(current+2)%4].1;
    println!("{} {} {} {}", x3, y3, x4, y4);
}