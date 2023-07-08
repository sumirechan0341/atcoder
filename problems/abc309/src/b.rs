use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        ann: [Chars; n]
    };
    for i in 0..n {
        for j in 0..n {
            if i == 0 {
                if j == 0 {
                    print!("{}", ann[i+1][j]);
                } else {
                    print!("{}", ann[i][j-1]);
                }
                
            } else if j == n-1 {
                print!("{}", ann[i-1][j]);
            } else if i == n-1 {
                print!("{}", ann[i][j+1]);
            } else if j == 0 {
                print!("{}", ann[i+1][j])
            } else {
                print!("{}", ann[i][j])
            }
        }
        println!("{}", "");
    }
}
