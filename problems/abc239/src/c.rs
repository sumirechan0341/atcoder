use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        xy1: (i32, i32),
        xy2: (i32, i32)
    };
    let p1s = genp(xy1);
    for p in genp(xy2) {
        if p1s.contains(&p) {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}

fn genp(p: (i32, i32)) -> Vec<(i32, i32)> {
    vec![(p.0+2, p.1+1), (p.0+1, p.1+2), (p.0+2, p.1-1), (p.0+1, p.1-2), (p.0-2, p.1-1), (p.0-1, p.1-2), (p.0-2, p.1+1), (p.0-1, p.1+2)]
}