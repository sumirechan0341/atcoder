use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        a: i32,
        b: i32,
        sdn: [(String, i32); n]
    };
    let mut ans = 0;
    for (s, d) in sdn {
        match &*s {
            "East" => {
                ans += d.max(a).min(b);
            },
            _ => {
                ans -= d.max(a).min(b);
            }
        }
    }
    if ans < 0 {
        println!("{} {}", "West", -ans);
    } else if ans > 0 {
        println!("{} {}", "East", ans);
    } else {
        println!("{}", 0);
    }
}