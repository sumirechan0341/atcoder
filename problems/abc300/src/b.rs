use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [Chars; h],
        mut b: [Chars; h],
    };
    for s in 0..h {
        for t in 0..w {
            let new_map = transform_map(&a, s, t, h, w);
            if is_same_map(&new_map, &b, h, w) {
                println!("{}", "Yes");
                return;
            }
        }
    }
    println!("{}", "No");
}

fn is_same_map(a: &Vec<Vec<char>>, b: &Vec<Vec<char>>, h: usize, w: usize) -> bool {
    for i in 0..h {
        for j in 0..w {
            if a[i][j] != b[i][j] {
                return false;
            }
        }
    }
    return true;
}

fn transform_map(original: &Vec<Vec<char>>, xshift: usize, yshift: usize, h: usize, w: usize) -> Vec<Vec<char>> {
    let mut res = original.to_vec();
    for i in 0..h {
        for j in 0..w {
            res[(i+xshift)%h][(j+yshift)%w] = original[i][j];
        }
    }
    return res;
}