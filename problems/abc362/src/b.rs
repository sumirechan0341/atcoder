use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: (i64, i64),
        b: (i64, i64),
        c: (i64, i64)
    };

    println!(
        "{}",
        if (a.0 - b.0) * (b.0 - c.0) + (a.1 - b.1) * (b.1 - c.1) == 0
            || (a.0 - b.0) * (a.0 - c.0) + (a.1 - b.1) * (a.1 - c.1) == 0
            || (a.0 - c.0) * (b.0 - c.0) + (a.1 - c.1) * (b.1 - c.1) == 0
        {
            "Yes"
        } else {
            "No"
        }
    );
}
