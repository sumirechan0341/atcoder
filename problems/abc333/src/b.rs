use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: String,
        t: String
    };
    let g1 = [
        "AB".to_string(),
        "BC".to_string(),
        "CD".to_string(),
        "DE".to_string(),
        "EA".to_string(),
        "BA".to_string(),
        "CB".to_string(),
        "DC".to_string(),
        "ED".to_string(),
        "AE".to_string(),
    ];
    if !(g1.contains(&s) ^ g1.contains(&t)) {
        println!("Yes");
    } else {
        println!("No");
    }
}
