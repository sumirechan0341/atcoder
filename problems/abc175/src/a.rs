use proconio::input;

pub fn main() {
    input !{
        s: String
    };
    println!("{}", match &*s {
        "SSS" => 0,
        "SSR" | "SRS" | "RSS" | "RSR" => 1,
        "SRR" | "RRS" => 2,
        _ => 3
    });
}