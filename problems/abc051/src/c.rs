use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        sx: i32,
        sy: i32,
        tx: i32,
        ty: i32
    };
    let mut ans = String::new();
    ans += &"R".repeat((tx-sx) as usize);
    ans += &"U".repeat((ty-sy) as usize);
    
    ans += &"L".repeat((tx-sx) as usize);
    ans += &"D".repeat((ty-sy) as usize);
    
    ans += &"D";
    ans += &"R".repeat((tx-sx+1) as usize);
    ans += &"U".repeat((ty-sy+1) as usize);
    ans += &"L";

    ans += &"U";
    ans += &"L".repeat((tx-sx+1) as usize);
    ans += &"D".repeat((ty-sy+1) as usize);
    ans += &"R";
    println!("{}", ans);
}