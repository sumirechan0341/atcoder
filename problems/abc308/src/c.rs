use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        abn: [(i64, i64); n]
    };
    let mut ans = abn.iter().enumerate().collect::<Vec<_>>();
    ans.sort_by(|c1, c2| ((c2.1).0*((c1.1).1+(c1.1).0)).cmp(&((c1.1).0*((c2.1).1+(c2.1).0))).then(c1.0.cmp(&c2.0)));
    for a in ans {
        println!("{}", a.0+1);
    }
}