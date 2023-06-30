use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        a2nm: [Chars; 2*n]  
    };
    let mut win = vec![];
    for i in 0..2*n {
        win.push((0, i));
    }
    for i in 0..m {
        win.sort_by(|&c1, &c2| c2.0.cmp(&c1.0).then(c1.1.cmp(&c2.1)));
        for j in 0..n {
            win[2*j].0 += jan(a2nm[win[2*j].1][i], a2nm[win[2*j+1].1][i]);
            win[2*j+1].0 += jan(a2nm[win[2*j+1].1][i], a2nm[win[2*j].1][i]);
        }
    }
    win.sort_by(|&c1, &c2| c2.0.cmp(&c1.0).then(c1.1.cmp(&c2.1)));
    for i in 0..2*n {
        println!("{}", win[i].1+1);
    }
}
fn jan(c1: char, c2: char) -> i32 {
    match (c1, c2) {
        ('G', 'G') => {
            0
        },
        ('G', 'C') => {
            1
        },
        ('G', 'P') => {
            0
        },
        ('C', 'G') => {
            0
        },
        ('C', 'C') => {
            0
        },
        ('C', 'P') => {
            1
        },
        ('P', 'G') => {
            1
        },
        ('P', 'C') => {
            0
        },
        _ => {
            0
        }
    }
}