use proconio::input;
use proconio::source::line::LineSource;
use std::io::{stdin, BufReader, stdout, Write};

pub fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        n: usize
    }

    let mut used = vec![false; 2*n+2];
    while true {
        for i in 1..=2*n+1 {
            if !used[i] {
                used[i] = true;
                println!("{}", i);
                stdout().flush().unwrap();
                break;
            }
        }
        input!{
            from &mut source,
            i: usize
        };
        if i == 0 {
            break;
        }
        used[i] = true;
    }
    return;
}