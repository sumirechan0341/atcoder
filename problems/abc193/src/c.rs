use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: i64
    };
    let mut count = 0;
    let mut used = vec![false; 100001];
    for i in 2..=100000 {
        if used[i as usize] {
            continue;
        }
        let mut j = i*i;
        while j <= n {
            if j < 100001 {
                used[j as usize] = true;
            }
            count += 1;
            j *= i;
        }
    }
    println!("{}", n-count);
}