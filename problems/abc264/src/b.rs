use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        r: usize,
        c: usize
    };
    let m = [
        ['■', '■', '■', '■', '■', '■', '■', '■', '■', '■', '■', '■', '■', '■', '■'],
        ['■', '□', '□', '□', '□', '□', '□', '□', '□', '□', '□', '□', '□', '□', '■'],
        ['■', '□', '■', '■', '■', '■', '■', '■', '■', '■', '■', '■', '■', '□', '■'],
        ['■', '□', '■', '□', '□', '□', '□', '□', '□', '□', '□', '□', '■', '□', '■'],
        ['■', '□', '■', '□', '■', '■', '■', '■', '■', '■', '■', '□', '■', '□', '■'],
        ['■', '□', '■', '□', '■', '□', '□', '□', '□', '□', '■', '□', '■', '□', '■'],
        ['■', '□', '■', '□', '■', '□', '■', '■', '■', '□', '■', '□', '■', '□', '■'],
        ['■', '□', '■', '□', '■', '□', '■', '□', '■', '□', '■', '□', '■', '□', '■'],
        ['■', '□', '■', '□', '■', '□', '■', '■', '■', '□', '■', '□', '■', '□', '■'],
        ['■', '□', '■', '□', '■', '□', '□', '□', '□', '□', '■', '□', '■', '□', '■'],
        ['■', '□', '■', '□', '■', '■', '■', '■', '■', '■', '■', '□', '■', '□', '■'],
        ['■', '□', '■', '□', '□', '□', '□', '□', '□', '□', '□', '□', '■', '□', '■'],
        ['■', '□', '■', '■', '■', '■', '■', '■', '■', '■', '■', '■', '■', '□', '■'],
        ['■', '□', '□', '□', '□', '□', '□', '□', '□', '□', '□', '□', '□', '□', '■'],
        ['■', '■', '■', '■', '■', '■', '■', '■', '■', '■', '■', '■', '■', '■', '■'],
    ];
    if m[r-1][c-1] == '■' {
        println!("{}", "black");
    } else {
        println!("{}", "white");
    }
}