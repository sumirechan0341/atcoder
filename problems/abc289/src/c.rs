use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: i32,
        m: usize,
    };
    let mut sm = vec![];
    for i in 0..m {
        input! {
            c: usize,
            ac: [i32; c]
        }
        sm.push(ac);
    }
    let mut count = 0;
    for i in 0..2_i32.pow(m as u32) {
        let mut bit = i;
        let mut sum = vec![];
        for j in 0..m {
            if bit & 1 == 1 {
                for s in &sm[j as usize] {
                    sum.push(*s);
                }
            }
            bit = bit >> 1;
        }
        let mut ok = true;
        for i in 1..=n {
            if !sum.contains(&i) {
                ok = false;
                break;
            }
        }
        if ok {
            count += 1;
        }
    }
    println!("{}", count);
}