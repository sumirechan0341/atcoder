use proconio::input;

pub fn main() {
    input! {
        c: char
    };
    println!("{}", if c.is_uppercase() {'A'} else {'a'});
}