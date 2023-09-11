use std::collections::HashMap;
use std::iter::FromIterator;
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: String
    };
    let rate = HashMap::<String, i32>::from_iter([
        ("tourist".to_string(), 3858),
        ("ksun48".to_string(), 3679),
        ("Benq".to_string(), 3658),
        ("Um_nik".to_string(), 3648),
        ("apiad".to_string(), 3638),
        ("Stonefeang".to_string(), 3630),
        ("ecnerwala".to_string(), 3613),
        ("mnbvmar".to_string(), 3555),
        ("newbiedmy".to_string(), 3516),
        ("semiexp".to_string(), 3481)]);
    println!("{}", rate.get(&s).unwrap());
}