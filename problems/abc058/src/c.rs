use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        sn: [Chars; n]
    };
    let mut alpha = vec![50; 26];
    for s in sn {
        let mut local = vec![0; 26];
        for c in s {
            local[(c as u8 - 97) as usize] += 1;
        }
        for i in 0..26 {
            alpha[i] = alpha[i].min(local[i]);
        }
    }
    for i in 0..26 {
        print!("{}", ((i+97 as u8) as char).to_string().repeat(alpha[i as usize]));
    }
    println!("{}", "");
}