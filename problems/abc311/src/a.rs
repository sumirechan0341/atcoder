use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        s: Chars
    };
    let mut state = vec![false ; 3];
    for i in 0..n {
        match s[i] {
            'A' => {
                state[0] = true;
            },
            'B' => {
                state[1] = true;
            },
            _ => {
                state[2] = true;
            }
        }
        if state.iter().all(|x| *x) {
            println!("{}", i+1);
            return;
        }
    }
}