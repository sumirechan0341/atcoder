use itertools::Itertools;
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: i32
    };
    let mut ans = vec![];
    for j in 3..10 {
        for i in 0..3_i32.pow(j) {
            let mut ii = i;
            let mut exist3 = false;
            let mut exist5 = false;
            let mut exist7 = false;
            let mut local = 0;
           
            for k in 0..j {
                if ii % 3 == 0 {
                    local += 10_i32.pow(k)*3;
                    exist3 = true;
                }
                if ii % 3 == 1 {
                    local += 10_i32.pow(k)*5;
                    exist5 = true;
                }
                if ii % 3 == 2 {
                    local += 10_i32.pow(k)*7;
                    exist7 = true;
                }
                ii /= 3;
            }
            if exist3 && exist5 && exist7 {
                ans.push(local);
            }
        }
    }    
    println!("{}", ans.iter().filter(|&x| x <= &n ).count());
}