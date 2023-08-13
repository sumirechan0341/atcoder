use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        k: usize,
        dk: [i32; k]
    };
    for i in 1..100000 {
        let s = i.to_string();
        let mut ok = true;
        for d in dk.iter().map(|x| x.to_string()) {
            if s.contains(&d) {
                ok = false;
                break;
            }
        }
        if ok && i >= n {
            println!("{}", i);
            return;
        }
    }
}