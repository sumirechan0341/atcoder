use proconio::{input, marker::{Chars, Usize1}};

pub fn main() {
    input! {
        n: usize,
        mut an: [i64; n],
        q: usize
    };
    let mut now = -1i64;
    let mut diff = vec![0; n];
    let mut ans = vec![];
    for _q in 0..q {
        input! {
            query_id: i32
        }
        if query_id == 1 {
            input! {
                x: i64
            }
            now = x;
            diff = vec![0; n];
        } else if query_id == 2 {
            input! {
                idx: Usize1,
                x: i64
            }
            diff[idx] += x;
        } else {
            input! {
                idx: Usize1
            }
            if now == -1 {
                ans.push((an[idx]+diff[idx]).to_string());
            } else {
                ans.push((diff[idx]+now).to_string());
            }
        }
    }
    println!("{}", ans.join("\n"));
}