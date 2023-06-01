use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        a: [[i32; n]; n],
        b: [[i32; n]; n]
    };
    for i in 0..4 {
        let new_matrix = rotate_matrix(&a, i, n);
        if is_valid_mat(&new_matrix, &b, n) {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");

}

fn rotate_matrix(a: &Vec<Vec<i32>>, rot_num: u32, n: usize) -> Vec<Vec<i32>> {
    let mut new_matrix = a.clone();
    for _r in 0..rot_num {
        let prev_matrix = new_matrix.clone();
        for i in 0..n {
            for j in 0..n {
                new_matrix[n-j-1][i] = prev_matrix[i][j];
            }
        }
    }
    return new_matrix;
}

fn is_valid_mat(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>, n: usize) -> bool {
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == 1 && a[i][j] != b[i][j] {
                return false;
            }
        }
    }
    return true;
}