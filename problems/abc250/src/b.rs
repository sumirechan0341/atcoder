use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        a: usize,
        b: usize
    };
    let mut field = vec![vec!['.'; b * n]; a * n];
    for i in 0..n {
        for j in 0..n {
            if (i % 2) ^ (j % 2) != 0 {
                for k in 0..a {
                    for l in 0..b {
                        field[i*a+k][j*b+l] = '#';
                    }
                }
            }
        }
    }
    for f in field {
        for a in f {
            print!("{}", a);
        }
        println!("{}", "");
    }
}