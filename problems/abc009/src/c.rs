use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        mut k: i64,
        s: Chars
    };
    if k == 0 {
        println!("{}", s.iter().collect::<String>());
        return;
    }
    let mut not_used = s.clone();
    not_used.sort();

    let mut t = vec![];

    for i in 0..n {
        for j in 0..not_used.len() {
            if s[i] != not_used[j] {
                let ss = s.clone()[i+1..].to_vec();
                let mut tt = not_used.clone();
                tt.remove(j);

                let mut extra = 0;

                let mut alpha = vec![0; 26];
                for l in 0..ss.len() {
                    alpha[(ss[l] as u8 - 97) as usize] += 1;
                }
                
                for l in 0..tt.len() {
                    if alpha[(tt[l] as u8 - 97) as usize] > 0 {
                        alpha[(tt[l] as u8 - 97) as usize] -= 1;
                    } else {
                        extra += 1;
                    }
                }
                if k - extra - 1 < 0 {
                    continue;
                }
                k -= 1;
                t.push(not_used[j]);
                not_used.remove(j);
                break;
            } else {
                t.push(not_used[j]);
                not_used.remove(j);
                break;
            }
        }
    }
    println!("{}", t.iter().collect::<String>());
}
