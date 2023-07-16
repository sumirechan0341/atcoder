use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        scm: [(usize, u32); m]
    };
    for i in 0..1000 {
        if i.to_string().chars().count() != n {
            continue;
        }
        let mut ok = true;
        for (s, c) in &scm {
            if i.to_string().chars().nth(s-1).unwrap().to_digit(10).unwrap() != *c {
                ok = false;
                break;
            }
        }
        if ok {
            println!("{}", i);
            return;
        }
    }
    println!("{}", -1);
}