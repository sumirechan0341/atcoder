use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        w: usize,
        h: usize,
        n: usize,
        xyan: [(usize, usize, usize); n]
    };
    let mut ans = vec![vec![true; w]; h];
   
    for (x, y, a) in xyan {
        match a {
            1 => {
                for i in 0..h {
                    for j in 0..x {
                        ans[i][j] = false;
                    }
                }
            },
            2 => {
                for i in 0..h {
                    for j in x..w {
                        ans[i][j] = false;
                    }
                }
            },
            3 => {
                for i in h-y..h {
                    for j in 0..w {
                        ans[i][j] = false;
                    }
                }
            },
            _ => {
                for i in 0..h-y {
                    for j in 0..w {
                        ans[i][j] = false;
                    }
                }
            }
        }
    }
    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            if ans[i][j] {
                count += 1;
            }
            // print!("{}", if ans[i][j] {'#'} else {'.'} );
        }
        // println!("{}", "");
    }
    println!("{}", count);

}