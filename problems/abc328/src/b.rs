use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        dn: [usize; n]
    };
    let mut ans = 0;
    for i in 1..=n {
        for d in 1..=dn[i-1] {
            let mut s = "".to_string();
            s += &i.to_string();
            s += &d.to_string();
            let x = s.chars().nth(0).unwrap();
            if s.chars().all(|y| x == y) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}