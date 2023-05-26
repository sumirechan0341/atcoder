use proconio::input;

pub fn main() {
    input! {
        s: String
    };
    println!("{}", if s <= "2019/04/30".to_string() { "Heisei" } else { "TBD" });

}