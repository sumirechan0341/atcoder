use proconio::{input, marker::Chars};
 
pub fn main() {
    input! {
        s: Chars,
        t: Chars
    };
    let mut strans_map = vec![100; 26];
    let mut ttrans_map = vec![100; 26];
    for i in 0..s.len() {
        let scode = (s[i] as u8 - 97) as usize;
        let tcode = (t[i] as u8 - 97) as usize;
        if strans_map[scode] == 100 && ttrans_map[tcode] == 100  {
            strans_map[scode] = tcode;
            ttrans_map[tcode] = scode;
        } else {
            if !(strans_map[scode] == tcode && ttrans_map[tcode] == scode) {
                println!("{}", "No");
                return;
            }
        }
    }
    println!("{}", "Yes");
}