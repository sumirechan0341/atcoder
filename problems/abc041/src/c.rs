use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i32; n]
    };
    let mut ani = an.iter().zip(1..).collect::<Vec<_>>();
    ani.sort_by_key(|x| x.0);
    ani.reverse();
    for (a, i) in ani {
        println!("{}", i);
    }
}