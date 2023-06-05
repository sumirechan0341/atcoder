use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        m: usize,
        mut an: [i32; n],
        bm: [i32; m]
    };
    for b in bm {
        let mut ng = true;
        for (i, a) in an.iter().enumerate() {
            if a == &b {
                an.remove(i);
                ng = false;
                break;
            }
        }
        if ng {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}