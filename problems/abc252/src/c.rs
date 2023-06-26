use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        sn: [Chars; n]
    };
    let mut ans = [0; 10];
    for i in 0..10_usize {
        let mut weight = [0; 10];
        for s in &sn {
            for j in 0..10 {
                if s[j].to_digit(10).unwrap() == i as u32 {
                    weight[j] += 1;
                }
            }
        }
        let mut local_ans = 0;
        for j in 0..10 {
            if weight[j] == 0 {
                continue;
            }
            if local_ans < (weight[j]-1) * 10 + j {
                local_ans = (weight[j]-1) * 10 + j;
            }
        }
        ans[i] = local_ans;
    }
    ans.sort();
    println!("{}", ans[0]);
}