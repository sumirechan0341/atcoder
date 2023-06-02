use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        t: usize,
    };
    let mut test_case: Vec<Vec<i32>> = vec![];
    for _i in 0..t {
        input! {
            mut _nlocal: usize,
            mut tlocal: [i32; _nlocal]
        }
        test_case.push(tlocal);
    }
    for test in test_case {
        println!("{}", test.iter().filter(|x| *x % 2 == 1).collect::<Vec<_>>().len());
    }
}