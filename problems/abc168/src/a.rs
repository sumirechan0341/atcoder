use proconio::input;

pub fn main() {
    input! {
        n: i32
    };
    let r = n % 10;
    println!("{}", match r {
      2 | 4 | 5 | 7 | 9 => "hon",
      0 | 1 | 6 | 8 => "pon",
      _ => "bon"
    } );
}