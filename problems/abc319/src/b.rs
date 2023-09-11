use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: i64
    };
    let mut js = vec![];
    for i in 1..=9 {
        if n%i == 0 {
            js.push(i);
        }
    }
    print!("1");
    js.reverse();
    for i in 1..=n {
        let mut result = -1;
        for j in &js {
            if i%(n/j)  == 0 {
                result = *j;
            } 
        }
        print!("{}", if result == -1 { '-' } else { result.to_string().chars().nth(0).unwrap() });
    }
}