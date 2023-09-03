use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        mut am: [usize; m]
    };
    am.push(0);
    am.push(n+1);
    am.sort();
    if m == 0 {
        println!("{}", 1);
        return;
    }
    let mut min_streak = !0;
    for i in 0..m+1 {
        if am[i+1]-am[i] == 1 {
            continue;
        }
        if min_streak > am[i+1]-am[i]-1 {
            min_streak = am[i+1]-am[i]-1;
        }
    }
    if min_streak == !0 {
        println!("{}", 0);
        return;
    }
    let mut ans = 0;
    for i in 0..m+1 {
        ans += (am[i+1]-am[i]-1+min_streak-1)/min_streak
    }
    println!("{}", ans);
}