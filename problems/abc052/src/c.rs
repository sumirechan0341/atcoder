use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: i64
    };
    let p = 1000000007;
    let mut factor = vec![0; 1001];
    for i in 1..=n {
        let mut remain = i;
        for j in 2.. {
            while remain % j == 0 {
                factor[j as usize] += 1;
                remain /= j;
            }
            if remain == 1 {
                break;
            }
        }
    }
    let mut ans: i64 = 1;
    for i in 1..1001 {
        ans *= factor[i]+1;
        ans %= p;
    }
    println!("{}", ans);
}