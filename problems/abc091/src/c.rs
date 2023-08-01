use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        abn: [(usize, usize); n],
        cdn: [(usize, usize); n]
    };
    let mut red_num = vec![vec![0; 201]; 201];
    for (a, b) in abn {
        for i in a+1..201 {
            for j in b+1..201 {
                red_num[i][j] += 1;
            }
        }
    }
    let mut count = 0;
    for (c, d) in cdn {
        if red_num[c][d] > 0 {
            count += 1;
            for i in 0..c {
                for j in 0..d {
                    red_num[i][j] -= 1; 
                }
            }
        }
    }
    for i in 0..10 {
        println!("{:?}", &red_num[i][0..10]);
    }
    println!("{}", count);
}