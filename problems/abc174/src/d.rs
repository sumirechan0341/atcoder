use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        cn: Chars
    };
    let mut r = n-1;
    let mut l = 0;
    let mut count = 0;
    while r > l {
        if cn[l] == 'W' && cn[r] == 'R' {
            count += 1;
            l += 1;
            r -= 1;
            continue;
        }
        if cn[r] == 'W' {
            r -= 1;
        }
        if cn[l] == 'R' {
            l += 1;
        }
        
    }
    println!("{}", count);
}