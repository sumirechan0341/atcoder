use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        mut lrm: [(usize, usize); m]
    };
    lrm.sort_by_key(|x| x.0);
    let lmax = lrm[m-1].0;
    lrm.sort_by_key(|x| x.1);
    let rmin = lrm[0].1;
    if rmin < lmax {
        println!("{}", 0);
        return;
    }
    println!("{}", rmin-lmax+1);
}