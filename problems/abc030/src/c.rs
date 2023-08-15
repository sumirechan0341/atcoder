use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        x: i64,
        y: i64,
        an: [i64; n],
        bm: [i64; m]
    };
    let mut ans = 0;
    let mut a_index = 0;
    let mut b_index = 0;
    let mut now = 0;
    while a_index < n {
        while a_index < n && an[a_index] < now {
            a_index += 1;
        }
        if a_index == n {
            break;
        }
        now = an[a_index] + x;
        while b_index < m && bm[b_index] < now {
            b_index += 1;
        }
        if b_index == m {
            break;
        } else {
            now = bm[b_index] + y;
            ans += 1;
        }
    }
    println!("{}", ans);
}