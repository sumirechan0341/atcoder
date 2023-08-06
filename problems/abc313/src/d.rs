use itertools::Itertools;
use proconio::{input, marker::Chars};
use proconio::source::line::LineSource;
use std::io::{stdin, BufReader, stdout, Write};
type VS = Vec<String>;

pub fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        n: usize,
        k: usize
    }
    
}