use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        sx: usize,
        sy: usize,
        tx: usize,
        ty: usize
    };
    let mut save = 0;
    let mut ssx = 0;
    let mut ssy = 0;
    let mut ttx = 0;
    let mut tty = 0;
    if sx > tx {
        ttx = sx;
        tty = sy;
        ssx = tx;
        ssy = ty;
    } else {
        ssx = sx;
        ssy = sy;
        ttx = tx;
        tty = ty;
    }
    let tate = ssy.max(tty) - ssy.min(tty);
    if ssx == ttx {
        println!("{}", tate);
        return;
    }
    if (ssx + ssy) % 2 == 0 {
        ssx += 1;
    }
    println!(
        "{}",
        tate + if ttx - ssx <= tate {
            0
        } else {
            (ttx + 1 - ssx - tate) / 2
        }
    );
}
