use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut scn: [(String, usize); n]
    };
    scn.sort();
    let s: usize = scn.iter().map(|x| x.1).sum::<usize>();
    println!("{}", scn[s % n].0);
}
