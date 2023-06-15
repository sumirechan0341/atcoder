use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars,
        t: i32
    };
    let mut dx: i32 = 0;
    let mut dy: i32 = 0;
    let mut wildcard = 0;
    for c in s {
        match c {
            'U' => {
                dy += 1;
            },
            'D' => {
                dy -= 1;
            },
            'R' => {
                dx += 1;
            },
            'L' => {
                dx -= 1;
            },
            _ => {
                wildcard += 1;
            }
        }
    }
    dx = dx.abs();
    dy = dy.abs();
    println!("{}", if t == 1 { dx + dy + wildcard } else { if dx+dy >= wildcard {(dx+dy)-wildcard} else { (wildcard-(dx+dy)) % 2 } } );
}