use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        x: i32,
        y: i32,
        an: [usize; n]
    };
    let offset = 10000;
    let mut dpx = vec![false; 20001];
    let mut dpy = vec![false; 20001];
    dpx[an[0]+offset] = true;
    dpy[0+offset] = true;
    for i in (2..n).step_by(2) {
        let mut local = vec![];
        for j in 0..dpx.len() {
            if dpx[j] {
                local.push(j+an[i]);
                local.push(j-an[i]);
                dpx[j] = false;
            }
        }
        for j in 0..local.len() {
            dpx[local[j]] = true;
        }
    }
    for i in (1..n).step_by(2) {
        let mut local = vec![];
        for j in 0..dpy.len() {
            if dpy[j] {
                local.push(j+an[i]);
                local.push(j-an[i]);
                dpy[j] = false;
            }
        }
        for j in 0..local.len() {
            dpy[local[j]] = true;
        }
    }
    if dpx[(x+10000) as usize] && dpy[(y+10000) as usize] {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}