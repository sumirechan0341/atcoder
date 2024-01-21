use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        s: Chars
    };
    let mut ans = vec![-1; n + 1];
    let mut ml = 0;
    let mut mr = n as i32;
    for i in 0..n {
        if s[i] == 'L' {
            ans[i] = mr;
            mr -= 1;
        } else {
            ans[i] = ml;
            ml += 1;
        }
    }
    ans[n] = ml;
    let mut ians = ans.iter().enumerate().collect::<Vec<_>>();
    ians.sort_by_key(|x| x.1);
    for i in 0..n + 1 {
        println!("{}", ians[i].0);
    }
}
