use proconio::input;
pub fn main() {
    input! {
        s: String
    };
    let ss = s.split(".").collect::<Vec<_>>();
    let a = ss[0];
    let b = ss[1].parse::<i32>().unwrap();
    println!("{}{}", a , if b < 3 { "-" } else if b < 7 { "" } else { "+" });
}