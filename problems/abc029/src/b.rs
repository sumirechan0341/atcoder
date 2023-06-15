use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s12: [Chars; 12]
    };
    println!("{}", s12.iter().filter(|x| x.contains(&'r')).into_iter().collect::<Vec<_>>().len());
}