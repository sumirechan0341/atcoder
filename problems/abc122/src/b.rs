use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        s: Chars
    };
    let mut max = 0;
    for start in 0..s.len() {
        let mut local_count = 0;
        for stride in 0..s.len()-start {
            match s[start+stride] {
                'A' | 'C' | 'G' | 'T' => {
                    local_count += 1;
                }
                _ => {
                    break;
                }
            } 
        }
        if max < local_count {
            max = local_count;
        }
    }
    println!("{}", max);
}