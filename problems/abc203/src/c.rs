use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        mut k: i64,
        mut abn: [(i64, i64); n]
    };
    abn.sort();
    let mut now = 0;
    for i in 0..n {
        if abn[i].0 > now + k {
            println!("{}", now+k);
            return;
        } else {
            k += now-abn[i].0+abn[i].1;
            now = abn[i].0;
        }
    }
    println!("{}", now+k);
}