use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars,
        t: Chars
    };
    let mut max_match = 0;
    for i in 0..s.len()-t.len()+1 {
        let mut count = 0;
        for j in 0..t.len() {
            if s[i+j] == t[j] {
                count += 1;
            }
        }
        if count > max_match {
            max_match = count;
        }
    }
    println!("{}", t.len()-max_match);
}