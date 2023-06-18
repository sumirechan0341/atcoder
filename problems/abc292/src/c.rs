use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: i32
    };
    let mut count: u32 = 0;
    for i in 1..n {
        let l = i;
        let m = n-i;
        let mut countl = 0;
        let mut countr = 0; 
        for j in 1.. {
            if j*j > l {
                break;
            }
            if l % j == 0 {
                if l / j != j {
                    countl += 2;
                } else {
                    countl += 1;
                }
            }
        }
        for j in  1.. {
            if j*j > m {
                break;
            }
            if m % j == 0 {
                if m / j != j {
                    countr += 2;
                } else {
                    countr += 1;
                }
            }
        }
        count += countl * countr;
    }
    println!("{}", count);
}