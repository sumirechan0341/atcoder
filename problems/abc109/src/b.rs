use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        wn: [Chars; n]
    };
    let mut prev_word = wn[0][0];
    let mut used = vec![];
    for w in &wn {
        if prev_word != w[0] || used.contains(&w) {
            println!("{}", "No");
            return;
        }
        used.push(w);
        prev_word = w[w.len()-1];
    }
    println!("{}", "Yes");
}