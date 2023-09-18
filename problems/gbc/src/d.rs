use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        snn: [[char; n]; n],
        tnn: [[char; n]; n]
    };
    for i in 0..4 {
        let ss = rotate(&snn, i);
        for j in 0..2 {
            let sss = flip_h(&ss, j);
            for k in 0..2 {
                let ssss = flip_v(&sss, k);
                for l in 0..2 {
                    let sssss = transpose(&ssss, l);
                    if is_same(&sssss, &tnn) {
                        println!("{}", "Yes");
                        return;
                    }
                }
            }
        }
    }
    println!("{}", "No");
}

fn rotate(s: &Vec<Vec<char>>, n: i32) -> Vec<Vec<char>> {
    let mut res = s.clone();
    for k in 0..n {
        let now = res.clone();
        for i in 0..s.len() {
            for j in 0..s.len() {
                res[i][j] = now[s.len()-1-j][i];
            }
        }
    }
    return res;
}

fn flip_h(s: &Vec<Vec<char>>, n: i32) -> Vec<Vec<char>> {
    if n%2 == 0 {
        return s.clone();
    }
    let mut res = vec![vec!['0'; s.len()]; s.len()];
    for i in 0..s.len() {
        for j in 0..s.len() {
            res[i][j] = s[s.len()-1-i][j];
        }
    }
    return res;
}

fn flip_v(s: &Vec<Vec<char>>, n: i32) -> Vec<Vec<char>> {
    if n%2 == 0 {
        return s.clone();
    }
    let mut res = vec![vec!['0'; s.len()]; s.len()];
    for i in 0..s.len() {
        for j in 0..s.len() {
            res[i][j] = s[i][s.len()-1-j];
        }
    }
    return res;
}

fn transpose(s: &Vec<Vec<char>>, n: i32) -> Vec<Vec<char>> {
    if n%2 == 0 {
        return s.clone();
    }
    let mut res = vec![vec!['0'; s.len()]; s.len()];
    for i in 0..s.len() {
        for j in 0..s.len() {
            res[i][j] = s[j][i];
        }
    }
    return res;
}

fn is_same(s: &Vec<Vec<char>>, t: &Vec<Vec<char>>) -> bool {
    for i in 0..s.len() {
        for j in 0..s.len() {
            if s[i][j] != t[i][j] {
                return false;
            }
        }
    }
    return true;
}
