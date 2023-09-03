use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        d: i64,
        p: i64,
        mut f: [i64; n]
    };
    f.sort();
    f.reverse();
    let mut total = 0;
    let mut local = 0;
    let mut count = 0;
    for i in 0..n {
        local += f[i];
        count += 1;
        if count == d {
            if local > p {
                total += 1;
            }
            local = 0;
            count = 0;
        }
    }
    if local > p {
        total += 1;
    }
    f.reverse();
    println!("{}", total*p+f[..(n as i64-total*d).max(0) as usize].iter().sum::<i64>());
}