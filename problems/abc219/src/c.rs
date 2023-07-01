use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        x: Chars,
        n: usize,
        mut sn: [Chars; n]
    };
    sn.sort_by(|s1, s2| {
        for i in 0..s1.len().min(s2.len()) {
            if s1[i] == s2[i] {
                continue;
            } else {
                return x.iter().position(|&c| s1[i] == c).cmp(&x.iter().position(|&c| s2[i] == c));
            }
        }
        return s1.len().cmp(&s2.len());
    });
    for s in sn {
        println!("{}", s.iter().collect::<String>());
    }
}