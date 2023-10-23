use proconio::{input, marker::Chars};
use itertools::Itertools;
pub fn main() {
    input! {
        n: usize,
        s: Chars
    };
    let mut cnt = vec![0; 10];
    for i in 0..n {
        cnt[s[i].to_digit(10).unwrap() as usize] += 1;
    }
    let mut ans = 0;
    for i in 0..3200000i64 {
        let mut local_cnt = vec![0; 10];
        let num_st =(i*i).to_string();
        for j in 0..num_st.len() {
            local_cnt[num_st.chars().nth(j).unwrap().to_digit(10).unwrap() as usize] += 1;
        }
        let mut ok = true;
        for j in 1..10 {
            if cnt[j] != local_cnt[j] {
                ok = false;
            }
        }
        if ok && local_cnt[0] <= cnt[0]  {
            ans += 1;
        }
    }
   println!("{}", ans);
}