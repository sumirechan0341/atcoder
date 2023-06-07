use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: i32,
        m: usize,
        t: i32,
        abm: [(i32, i32); m]
    };
    let mut d = n;
    let mut time = 0;
    for ab in abm {
        d -= ab.0 - time;
        if d <= 0 {
            println!("{}", "No");
            return;
        }
        d += ab.1 - ab.0;
        if d > n {
            d = n;
        }
        time = ab.1;
    }
    if d > t - time {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}