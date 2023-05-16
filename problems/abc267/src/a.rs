use proconio::input;

pub fn main() {
    input! {
        s: String
    }
    let day = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    println!("{}", 5 - day.iter().position(|d| d == &&s).unwrap());
}