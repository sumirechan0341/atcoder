use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        an: [usize; n],
        bm: [usize; m],
    };
    let mut c = an.clone();
    c.extend(bm);
    c.sort();
    for i in 0..n + m - 1 {
        if an.contains(&c[i]) && an.contains(&c[(i + 1)]) {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}
