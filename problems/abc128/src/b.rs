use proconio::{input, marker::Chars};
use itertools::zip;
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        mut spn: [(String, i32); n]
    };
    let mut spni = zip(spn, 1..).collect::<Vec<_>>();
    spni.sort_by(|sp1, sp2| (sp1.0).0.cmp(&(sp2.0).0).then((sp2.0).1.cmp(&(sp1.0).1)));
    for spi in spni {
        println!("{}", spi.1);
    }
}